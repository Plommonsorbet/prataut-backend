use futures::executor::LocalPool;
use futures_util::{future::FutureExt, stream::StreamExt, task::LocalSpawnExt};
use lapin::{
    self, auth::Credentials, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
    types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    ConsumerDelegate,
};
use log::{error, info};
use redis;
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

pub struct Server {
    pub out: Sender,
    pub count: Rc<Cell<u32>>,
}

fn spawn_redis_subscriber(sender: &Sender) {
    // Clone sender so the message sent can be acked.
    let sender = sender.clone();

    thread::spawn(move || {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();
        let mut pubsub = con.as_pubsub();
        let channel_id = format!("ws_conn_{}", sender.connection_id());
        pubsub.subscribe("cancel_worker").unwrap();
        pubsub.subscribe(&channel_id).unwrap();
        loop {
            let msg = pubsub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();

            if dbg!(&payload == &channel_id) && dbg!(msg.get_channel_name() == "cancel_worker") {
                info!("exiting worker for sender id: '{}'", channel_id);
                pubsub.punsubscribe("*").unwrap();
                break;
            };
            sender.send(Message::text(payload.to_string()));
            info!("channel '{}': {}", msg.get_channel_name(), &payload);
        }
    });

    info!("worker closed!");
}

impl Handler for Server {
    //
    fn on_request(&mut self, req: &Request) -> ws::Result<(Response)> {
        // Using multiple handlers is better (see router example)

        match req.resource() {
            // The default trait implementation
            "/ws" => Response::from_request(req),
            "/" => Ok(Response::new(200, "OK", fs::read("client.html").unwrap())),

            // Create a custom response
            //"/" => Ok(Response::new(200, "OK", INDEX_HTML.to_vec())),
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> ws::Result<()> {
        self.count.set(self.count.get() + 1);

        let number_of_connection = self.count.get();
        info!("{:?}", handshake.request);

        let open_message = format!(
            "{} entered and the number of live connections is {}",
            &handshake.peer_addr.unwrap(),
            &number_of_connection
        );

        //        //dbg!(self.out.connection_id());
        spawn_redis_subscriber(&self.out);

        info!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
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
        info!("{:?}", &msg);
        self.out.broadcast(msg)
    }
}
