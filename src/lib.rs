use futures::executor::LocalPool;
use futures_util::{future::FutureExt, stream::StreamExt, task::LocalSpawnExt};
use lapin::{
    self, auth::Credentials, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
    types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    ConsumerDelegate,
};
use log::{error, info};
use redis::{self, Commands};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::Cell;
use std::fs;
use std::rc::Rc;
use std::str;
use std::thread;
use std::time::Duration;
//use std::{thread, time};
use async_std::task;
use std::sync::mpsc;
use uuid::Uuid;
use ws::{self, listen, CloseCode, Handler, Handshake, Message, Request, Response, Sender};

#[derive(Serialize, Deserialize, Debug)]
enum QueueType {
    Listener,
    Venter,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientRequest {
    Queue { queue_type: QueueType },
    Message { msg: String },
}

pub struct Server {
    pub out: Sender,
    pub count: Rc<Cell<u32>>,
}

fn new_redis_connection() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    con
}

fn spawn_redis_subscriber(sender: &Sender, uid: String, mut con: redis::Connection) {
    // Clone sender so the message sent can be acked.
    let sender = sender.clone();

    thread::spawn(move || {
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe("cancel_worker").unwrap();
        pubsub.subscribe(&uid).unwrap();
        loop {
            let msg = pubsub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();

            if dbg!(&payload == &uid) && dbg!(msg.get_channel_name() == "cancel_worker") {
                info!("exiting worker for sender id: '{}'", uid);
                pubsub.punsubscribe("*").unwrap();
                break;
            };
            sender.send(Message::text(payload.to_string()));
            info!("channel '{}': {}", msg.get_channel_name(), &payload);
        }
    });
}

fn redis_publish(msg: &str, channel: &str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let _: () = con.publish(channel, msg).unwrap();
    Ok(())
}

//fn fetch_an_integer(con: &mut redis::Connection) -> redis::RedisResult<()> {
//
//    // connect to redis
//    //let client = redis::Client::open("redis://127.0.0.1/")?;
//    //let mut con = client.get_connection()?;
//    // throw away the result, just make sure it does not fail
//    //let _: () = con.set("my_key", 42)?;
//    // read back the key and return it.  Because the return value
//    // from the function is a result for integer this will automatically
//    // convert into one.
//    //con.get("my_key")
//}
fn get_uid(session_id: u32) -> String {
    let mut con = new_redis_connection();
    let uid = con.get(format!("{}_uuid", &session_id)).unwrap();
    uid
}

fn add_user_to_queue(queue: &str, uid: &str) -> redis::RedisResult<()> {
    let mut con = new_redis_connection();
    let _: () = con.lpush(queue, uid.to_string()).unwrap();
    Ok(())
}
//fn list_len(queue: &str) -> i32 {
//    let mut con = new_redis_connection();
//    let list_len: i32 = con.lpush(queue, uid.to_string()).unwrap();
//    list_len
//}

impl Handler for Server {
    //
    fn on_request(&mut self, req: &Request) -> ws::Result<(Response)> {
        // Using multiple handlers is better (see router example)

        match req.resource() {
            // The default trait implementation
            x if dbg!(&x[..3]) == "/ws" => Response::from_request(req),
            //"/ws" => Response::from_request(req),
            "/" => Ok(Response::new(200, "OK", fs::read("client.html").unwrap())),

            // Create a custom response
            //"/" => Ok(Response::new(200, "OK", INDEX_HTML.to_vec())),
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> ws::Result<()> {
        self.count.set(self.count.get() + 1);

        let uid = handshake.request.resource()[4..].to_string();
        let mut con = new_redis_connection();
        let _: () = con
            .set(
                format!("{}_session_id", &uid),
                self.out.connection_id().to_string(),
            )
            .unwrap();
        let _: () = con
            .set(
                format!("{}_uuid", self.out.connection_id()),
                uid.to_string(),
            )
            .unwrap();
        let number_of_connection = self.count.get();
        info!("{:?}", handshake.request);

        let open_message = format!(
            "{} entered and the number of live connections is {}",
            &handshake.peer_addr.unwrap(),
            &number_of_connection
        );

        //        //dbg!(self.out.connection_id());
        spawn_redis_subscriber(&self.out, uid, new_redis_connection());

        info!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        redis_publish(
            &format!("ws_conn_{}", self.out.connection_id()),
            "cancel_worker",
        )
        .unwrap();

        let uid = get_uid(self.out.connection_id());
        let mut con = new_redis_connection();

        let _: () = con.lrem("listeners", 0, &uid).unwrap();
        let _: () = con.lrem("venters", 0, &uid).unwrap();
        match code {
            CloseCode::Normal => info!("The client is done with the connection."),
            CloseCode::Away => info!("The client is leaving the site."),
            CloseCode::Abnormal => {
                error!("Closing handshake failed! Unable to obtain closing status from client.")
            }
            _ => error!("The client encountered an error: {}", reason),
        }
        self.count.set(self.count.get() - 1)
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        // Broadcast to all connections
        info!("{:?}", &msg.as_text());
        let uid = get_uid(self.out.connection_id());

        let mut con = new_redis_connection();
        let req: ClientRequest = serde_json::from_str(&msg.as_text().unwrap()).unwrap();
        match req {
            ClientRequest::Queue { queue_type } => match queue_type {
                QueueType::Listener => {
                    add_user_to_queue("listeners", &uid);
                    let li: redis::RedisResult<String> = con.rpop("venters");
                    if let Ok(venter) = li {
                        info!("paired up {} and {}", venter, uid);
                        self.out
                            .send(format!("You've entered a chat room with: {}", venter));
                        redis_publish(
                            &format!("You've entered a chat room with: {}", &uid),
                            &venter.to_string(),
                        )
                        .unwrap();
                        let _: () = con.set(format!("{}_chat_with", venter), &uid).unwrap();
                        let _: () = con.set(format!("{}_chat_with", &uid), venter).unwrap();
                    };
                    info!("listener joined queue!");
                }
                QueueType::Venter => {
                    add_user_to_queue("venters", &uid);
                    info!("venter joined queue!");
                    let li: redis::RedisResult<String> = con.rpop("listeners");
                    if let Ok(listener) = li {
                        info!("paired up {} and {}", listener, &uid);
                        self.out
                            .send(format!("You've entered a chat room with: {}", listener));

                        redis_publish(
                            &format!("You've entered a chat room with: {}", &uid),
                            &listener.to_string(),
                        )
                        .unwrap();
                        let _: () = con.set(format!("{}_chat_with", listener), &uid).unwrap();
                        let _: () = con.set(format!("{}_chat_with", &uid), listener).unwrap();
                    };
                }
            },
            ClientRequest::Message { msg } => {
                info!("incoming msg: {}", msg);
                let chatting_with: redis::RedisResult<String> =
                    con.get(format!("{}_chat_with", uid));
                if let Ok(to) = chatting_with {
                    info!("publishing msg: {}", msg);
                    redis_publish(&format!("from {}: {}", &uid, &msg), &to.to_string()).unwrap();
                }
            }
        };
        Ok(())
    }
}
