use lapin::{
    self,
    auth::Credentials, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
    types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use futures_util::{future::FutureExt, stream::StreamExt, task::LocalSpawnExt};
use log::info;
use std::str;
use futures::executor::LocalPool;

//pub async fn sender(addr: &str, payload: &str) {
pub async fn sender(payload: &str) {
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default())
        .await
        .expect("connection error");

    info!("CONNECTED");

    //send channel
    let channel_a = conn.create_channel().await.expect("create_channel");
    info!("[{}] state: {:?}", line!(), conn.status().state());

    //create the hello queue
    let queue = channel_a
        .queue_declare(
            "my_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("queue_declare");
    info!("[{}] state: {:?}", line!(), conn.status().state());
    info!("[{}] declared queue: {:?}", line!(), queue);

    let queue = channel_a
        .confirm_select(ConfirmSelectOptions::default())
        .await
        .expect("confirm_select");
    info!("[{}] state: {:?}", line!(), conn.status().state());
    info!("Enabled publisher-confirms: {:?}", queue);

    info!("[{}] state: {:?}", line!(), conn.status().state());

    info!("will publish");
    //let payload = b"Hello world!";
    let confirm = channel_a
        .basic_publish(
            "",
            "my_queue",
            BasicPublishOptions::default(),
            payload.as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .await
        .expect("basic_publish")
        .await // Wait for this specific ack/nack
        .expect("publisher-confirms");
    assert_eq!(confirm, Confirmation::Ack);
    info!("[{}] state: {:?}", line!(), conn.status().state());
}


pub async fn listener(addr: String) -> String {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");

        //receive channel
        let channel = conn.create_channel().await.expect("create_channel");
        info!("[{}] state: {:?}", line!(), conn.status().state());

        let queue = channel
            .queue_declare(
                "my_queue",
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
                "my_queue",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        info!("[{}] state: {:?}", line!(), conn.status().state());

        for delivery in consumer {
            if let Ok(delivery) = delivery {
		info!("received message: {}", str::from_utf8(&delivery.data).unwrap());
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    String::from("From listener...")

}

pub fn pub_sub(addr: &str, msg: &str) -> lapin::Result<()>{
    //std::env::set_var("RUST_LOG", "info");

    //env_logger::init();

    //let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let mut executor = LocalPool::new();
    let spawner = executor.spawner();

    executor.run_until(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

        info!("CONNECTED");

        let channel_a = conn.create_channel().await?;
        let channel_b = conn.create_channel().await?;

        let queue = channel_a
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("Declared queue {:?}", queue);

        let consumer = channel_b
            .clone()
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        let _consumer = spawner.spawn_local(async move {
            info!("will consume");
            consumer
                .for_each(move |delivery| {
                    let delivery = delivery.expect("error caught in in consumer");
                    channel_b
                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                        .map(|_| ())
                })
                .await
        });

        let payload = b"Hello world!";

        loop {
            let confirm = channel_a
                .basic_publish(
                    "",
                    "hello",
                    BasicPublishOptions::default(),
                    payload.to_vec(),
                    BasicProperties::default(),
                )
                .await?
                .await?;
            assert_eq!(confirm, Confirmation::NotRequested);
        }
    })
}
