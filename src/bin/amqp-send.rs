use futures::executor::LocalPool;
//use lapin::{
//    auth::Credentials, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
//    types::FieldTable, BasicProperties, Connection, ConnectionProperties,
//};
//use log::info;
use prataut_backend::sender;

fn main() {
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());

    let mut executor = LocalPool::new();

    executor.run_until(sender(&addr, "This is my message from client."));
}
//async fn sender(addr: &str, payload: &str) {
//    let conn = Connection::connect(&addr, ConnectionProperties::default())
//        .await
//        .expect("connection error");
//
//    info!("CONNECTED");
//
//    //send channel
//    let channel_a = conn.create_channel().await.expect("create_channel");
//    info!("[{}] state: {:?}", line!(), conn.status().state());
//
//    //create the hello queue
//    let queue = channel_a
//        .queue_declare(
//            "my_queue",
//            QueueDeclareOptions::default(),
//            FieldTable::default(),
//        )
//        .await
//        .expect("queue_declare");
//    info!("[{}] state: {:?}", line!(), conn.status().state());
//    info!("[{}] declared queue: {:?}", line!(), queue);
//
//    let queue = channel_a
//        .confirm_select(ConfirmSelectOptions::default())
//        .await
//        .expect("confirm_select");
//    info!("[{}] state: {:?}", line!(), conn.status().state());
//    info!("Enabled publisher-confirms: {:?}", queue);
//
//    info!("[{}] state: {:?}", line!(), conn.status().state());
//
//    info!("will publish");
//    //let payload = b"Hello world!";
//    let confirm = channel_a
//        .basic_publish(
//            "",
//            "my_queue",
//            BasicPublishOptions::default(),
//            payload.as_bytes().to_vec(),
//            BasicProperties::default(),
//        )
//        .await
//        .expect("basic_publish")
//        .await // Wait for this specific ack/nack
//        .expect("publisher-confirms");
//    assert_eq!(confirm, Confirmation::Ack);
//    info!("[{}] state: {:?}", line!(), conn.status().state());
//}
