//use futures::executor::LocalPool;
//use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
//use log::info;
//use std::str;
use prataut_backend::listener;

use futures::executor::LocalPool;
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();


    let mut executor = LocalPool::new();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
    listener();
    //executor.run_until(listener(addr.to_string()));
}

//async fn listener(addr: String) {
//        let conn = Connection::connect(&addr, ConnectionProperties::default())
//            .await
//            .expect("connection error");
//
//        info!("CONNECTED");
//
//        //receive channel
//        let channel = conn.create_channel().await.expect("create_channel");
//        info!("[{}] state: {:?}", line!(), conn.status().state());
//
//        let queue = channel
//            .queue_declare(
//                "my_queue",
//                QueueDeclareOptions::default(),
//                FieldTable::default(),
//            )
//            .await
//            .expect("queue_declare");
//        info!("[{}] state: {:?}", line!(), conn.status().state());
//        info!("declared queue {:?}", queue);
//
//        info!("will consume");
//        let consumer = channel
//            .basic_consume(
//                "my_queue",
//                "my_consumer",
//                BasicConsumeOptions::default(),
//                FieldTable::default(),
//            )
//            .await
//            .expect("basic_consume");
//
//        info!("[{}] state: {:?}", line!(), conn.status().state());
//
//        for delivery in consumer {
//            if let Ok(delivery) = delivery {
//		info!("received message: {}", str::from_utf8(&delivery.data).unwrap());
//                channel
//                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
//                    .await
//                    .expect("basic_ack");
//            }
//        }
//
//}
