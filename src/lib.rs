use futures::executor::LocalPool;
use futures_util::{future::FutureExt, stream::StreamExt, task::LocalSpawnExt,};
use lapin::{
    self, auth::Credentials, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
    types::FieldTable, BasicProperties, Connection, ConnectionProperties, ConsumerDelegate, Channel
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::Cell;
use std::fs;
use std::rc::Rc;
use std::str;
use std::{thread, time};
use uuid::Uuid;
use ws::{self, listen, Handler, Handshake, Message, Request, Response, Sender};
use std::time::Duration;

use async_std::task;

#[derive(Clone, Debug)]
struct Subscriber {
    channel: Channel,
}

impl ConsumerDelegate for Subscriber {
    fn on_new_delivery(&self, delivery: DeliveryResult) {
        if let Ok(Some(delivery)) = delivery {
            self.channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .wait()
                .expect("basic_ack");
        }
    }
}

//pub fn listener() {

    //let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
    //let conn = Connection::connect(&addr, ConnectionProperties::default())
    //    .wait()
    //    .expect("connection error");

    //info!("CONNECTED");

    //let channel_a = conn.create_channel().wait().expect("create_channel");
    ////let channel_b = conn.create_channel().wait().expect("create_channel");

    //let queue = channel_a
    //    .queue_declare(
    //        "hello",
    //        QueueDeclareOptions::default(),
    //        FieldTable::default(),
    //    )
    //    .wait()
    //    .expect("queue_declare");

    ////info!("Declared queue {:?}", queue);

    //info!("will consume");
    //let consumer = channel_a
    //    .clone()
    //    .basic_consume(
    //        "",
    //        "hello",
    //        BasicConsumeOptions::default(),
    //        FieldTable::default(),
    //    )
    //    .wait()
    //    .expect("basic_consume")
    //    .set_delegate(Box::new(Subscriber {
    //        channel: channel_a.clone(),
    //    }));

    //dbg!(subscriber);

    //let payload = b"Hello world!";

    //loop {
    //    let confirm = channel_a
    //        .basic_publish(
    //            "",
    //            "hello",
    //            BasicPublishOptions::default(),
    //            payload.to_vec(),
    //            BasicProperties::default(),
    //        )
    //        .wait()
    //        .expect("basic_publish")
    //        .wait()
    //        .expect("publisher-confirms");
    //    assert_eq!(confirm, Confirmation::NotRequested);
    //}
//}



// This can be read from a file
//static INDEX_HTML: &'static [u8] = br#"
//<!DOCTYPE html>
//<html>
//	<head>
//		<meta charset="utf-8">
//	</head>
//	<body>
//      <pre id="messages"></pre>
//			<form id="form">
//				<input type="text" id="msg">
//				<input type="submit" value="Send">
//			</form>
//      <script>
//        var socket = new WebSocket("ws://" + window.location.host + "/ws");
//        socket.onmessage = function (event) {
//          var messages = document.getElementById("messages");
//          messages.append(event.data + "\n");
//        };
//        var form = document.getElementById("form");
//        form.addEventListener('submit', function (event) {
//          event.preventDefault();
//          var input = document.getElementById("msg");
//          socket.send(input.value);
//          input.value = "";
//        });
//		</script>
//	</body>
//</html>
//    "#;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientRequest {
    Poll,
    Send { message: String },
}

// Server web application handler
pub struct Server {
    pub out: Sender,
    pub count: Rc<Cell<u32>>,
}

impl Handler for Server {
    //

    //dbg!(self.out.token());
    //dbg!(self.out.token(),self.out.connection_id());
    fn on_request(&mut self, req: &Request) -> ws::Result<(Response)> {
        // Using multiple handlers is better (see router example)
        let new_uuid = Uuid::new_v4();
        match req.resource() {
            // The default trait implementation
            //"/ws" => Response::from_request(req),

            // Create a custom response
            //"/" => Ok(Response::new(200, "OK", fs::read("client.html").unwrap())),

            _ => Response::from_request(req),

            //_ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
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

        println!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        info!("Received message {}", &msg);
        if let Ok(text) = msg.as_text() {
            let client_request: serde_json::Result<ClientRequest> = serde_json::from_str(text);
            if let Ok(req) = client_request {
                match req {
                    ClientRequest::Poll => {
                        info!(
                            "POLLING! token: {:?}, conn_id: {}",
                            self.out.token(),
                            self.out.connection_id()
                        );
                        match self.out.connection_id() {
                            x if x == 1 => {
                                info!("conn_id: 1");
                                self.out.send(Message::text("Message for conn_id: 1"));
                            }
                            x if x == 2 => {
                                info!("conn_id: 2");
                                self.out.send(Message::text("Message for conn_id: 2"));
                            }
                            x if x == 3 => {
                                info!("conn_id: 3");
                                self.out.send(Message::text("Message for conn_id: 3"));
                            }
                            x if x == 4 => {
                                info!("conn_id: 4");
                                self.out.send(Message::text("Message for conn_id: 4"));
                            }
                            x if x == 5 => {
                                info!("conn_id: 5");
                                self.out.send(Message::text("Message for conn_id: 5"));

                            }
                            _ => info!("conn id is higher than 5"),
                        };
                        self.out.send(Message::text("POLLING."));
                    }
                    ClientRequest::Send { message } => {
                        info!(
                            "SEND MSG! token: {:?}, conn_id: {}\nmsg: {}",
                            self.out.token(),
                            self.out.connection_id(),
                            message
                        );
                        self.out.broadcast(Message::text(message));
                    }
                }
            } else {
                error!(
                    "Message can't be converted to json! token: {:?}, conn_id: {}\nreq: {:?}",
                    self.out.token(),
                    self.out.connection_id(),
                    client_request
                );
            }
        } else {
            error!(
                "Message is not utf8! token: {:?}, conn_id: {}",
                self.out.token(),
                self.out.connection_id()
            );
        };
        //let chat_message: ChatMessage = serde_json::from_str(&msg.as_text()?)?;

        info!("leaving send!");
        Ok(())
        //let addr = std::env::var("AMQP_ADDR")
        //    .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());

        //let result = executor.run_until(listener(addr.to_string()));
        //let addr =
        //    std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

        //Ok(())
        //match &msg.as_text().unwrap().to_string() {
        //    x if x.clone() == "i_wanna_listen".to_string() => {
        //	dbg!("this be awesum");
        //	for i in 1..15 {

        //	    let ten_millis = time::Duration::from_millis(1000);
        //	    let now = time::Instant::now();
        //
        //	    thread::sleep(ten_millis);
        //	    self.out.send(format!("I: {}", i));
        //	};

        //    }
        //    _ => ()
        //};
        //let chat_message: ChatMessage = serde_json::from_str(&msg.as_text().unwrap()).expect("not in json format");
        //dbg!(&msg.as_text());
        //dbg!(self.out.connection_id());

        //let addr = std::env::var("AMQP_ADDR")
        //    .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
        // Broadcast to all connections
        //self.out.broadcast(msg);

        //self.out.send("this is a private message sent only to this client.");
        //for i in 1..100 {
        //    self.out.send(format!("I: {}", i));
        //};
        //dbg!(self.out.connection_id());
        //executor.run_until(sender(&addr, "This is my message from client."));
        //executor.run_until(sender("This is my message from client."));
        //let result = executor.run_until(listener(addr.to_string()));
        //dbg!(result);
        //self.out.send();
        // Broadcast to all connections
        //self.out.broadcast(msg)
        //let mut executor = LocalPool::new();
        //pub_sub(&addr, "HIIIII").unwrap();
        //match msg {
        //    Text { msg } => {
        //    let chat_message: ChatMessage = serde_json::from_str(&msg).expect("not in json format");
        //	dbg!(chat_message);
        //    }
        //    Binary { bin } => {
        //	dbg!("in binary format");
        //    }
        //};
    }
}

pub fn sender() {

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default())
        .wait()
        .expect("connection error");

    info!("CONNECTED");

    let channel_a = conn.create_channel().wait().expect("create_channel");
    ////let channel_b = conn.create_channel().wait().expect("create_channel");

    //let queue = channel_a
    //    .queue_declare(
    //        "hello",
    //        QueueDeclareOptions::default(),
    //        FieldTable::default(),
    //    )
    //    .wait()
    //    .expect("queue_declare");

    //info!("Declared queue {:?}", queue);

    //info!("will consume");
    //channel_b
    //    .clone()
    //    .basic_consume(
    //        "hello",
    //        "my_consumer",
    //        BasicConsumeOptions::default(),
    //        FieldTable::default(),
    //    )
    //    .wait()
    //    .expect("basic_consume")
    //    .set_delegate(Box::new(Subscriber {
    //        channel: channel_b.clone(),
    //    }));

    let payload = b"Hello world!";
    let confirm = channel_a
        .basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            payload.to_vec(),
            BasicProperties::default(),
        )
        .wait()
        .expect("basic_publish")
        .wait()
        .expect("publisher-confirms");

    //loop {
    //    let confirm = channel_a
    //        .basic_publish(
    //            "",
    //            "hello",
    //            BasicPublishOptions::default(),
    //            payload.to_vec(),
    //            BasicProperties::default(),
    //        )
    //        .wait()
    //        .expect("basic_publish")
    //        .wait()
    //        .expect("publisher-confirms");
    //    assert_eq!(confirm, Confirmation::NotRequested);
    //}
}

pub fn listener () {
    let mut executor = LocalPool::new();
        let addr = std::env::var("AMQP_ADDR")
            .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());

    executor.run_until(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");

        //receive channel
        let channel = conn.create_channel().await.expect("create_channel");
        info!("[{}] state: {:?}", line!(), conn.status().state());

        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue_declare");
        info!("[{}] state: {:?}", line!(), conn.status().state());
        info!("declared queue {:?}", queue);

        info!("will consume");
        let consumer = channel
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");
        info!("[{}] state: {:?}", line!(), conn.status().state());

        for delivery in consumer {
            info!("received message: {:?}", delivery);
            if let Ok(delivery) = delivery {
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
	info!("does it reach end");
    })
}
