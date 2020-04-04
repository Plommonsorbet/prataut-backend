
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
//          var obj = new Object();
//          obj.msg = input.value;
//          obj.other_field = "lol'd";
//          socket.send(JSON.stringify(obj));
//          input.value = "";
//        });
//		</script>
//	</body>
//</html>
//    "#;
//pub async fn sender(payload: &str) {
//pub async fn sender(addr: &str, payload: &str) {
//    let addr = std::env::var("AMQP_ADDR")
//        .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
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
//
//pub async fn listener(addr: String) -> String {
//    let conn = Connection::connect(&addr, ConnectionProperties::default())
//        .await
//        .expect("connection error");
//
//    info!("CONNECTED");
//
//    //receive channel
//    let channel = conn.create_channel().await.expect("create_channel");
//    info!("[{}] state: {:?}", line!(), conn.status().state());
//
//    let queue = channel
//        .queue_declare(
//            "my_queue",
//            QueueDeclareOptions::default(),
//            FieldTable::default(),
//        )
//        .await
//        .expect("queue_declare");
//    info!("[{}] state: {:?}", line!(), conn.status().state());
//    info!("declared queue {:?}", queue);
//
//    info!("will consume");
//    let consumer = channel
//        .basic_consume(
//            "my_queue",
//            "my_consumer",
//            BasicConsumeOptions::default(),
//            FieldTable::default(),
//        )
//        .await
//        .expect("basic_consume");
//
//    info!("[{}] state: {:?}", line!(), conn.status().state());
//
//    for delivery in consumer {
//        if let Ok(delivery) = delivery {
//            let mesg = str::from_utf8(&delivery.data).unwrap();
//            info!("received message: {}", &mesg);
//            channel
//                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
//                .await
//                .expect("basic_ack");
//            return mesg.to_string();
//        }
//    }
//    String::from("From listener...")
//}
//
//pub fn pub_sub(addr: &str, msg: &str) -> lapin::Result<()> {
//    //std::env::set_var("RUST_LOG", "info");
//
//    //env_logger::init();
//
//    //let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
//    let mut executor = LocalPool::new();
//    let spawner = executor.spawner();
//
//    executor.run_until(async {
//        let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
//
//        info!("CONNECTED");
//
//        let channel_a = conn.create_channel().await.unwrap();
//        let channel_b = conn.create_channel().await.unwrap();
//
//        let queue = channel_a
//            .queue_declare(
//                "hello",
//                QueueDeclareOptions::default(),
//                FieldTable::default(),
//            )
//            .await?;
//
//        info!("Declared queue {:?}", queue);
//
//        let consumer = channel_b
//            .clone()
//            .basic_consume(
//                "hello",
//                "my_consumer",
//                BasicConsumeOptions::default(),
//                FieldTable::default(),
//            )
//            .await?;
//        let _consumer = spawner.spawn_local(async move {
//            info!("will consume");
//            consumer
//                .for_each(move |delivery| {
//                    let delivery = delivery.expect("error caught in in consumer");
//
//                    channel_b
//                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
//                        .map(|_| ())
//                })
//                .await
//        });
//
//        //let payload = b"Hello world!";
//
//        //loop {
//        //    let confirm = channel_a
//        //        .basic_publish(
//        //            "",
//        //            "hello",
//        //            BasicPublishOptions::default(),
//        //            payload.to_vec(),
//        //            BasicProperties::default(),
//        //        )
//        //        .await?
//        //        .await?;
//        //    assert_eq!(confirm, Confirmation::NotRequested);
//        //}
//    });
//}


//impl ConsumerDelegate for Subscriber {
//    fn on_new_delivery(&self, delivery: DeliveryResult) {
//        if let Ok(Some(delivery)) = delivery {
//            self.channel
//                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
//                .wait()
//                .expect("basic_ack");
//        }
//    }
//}

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


//pub fn sender() {
//
//    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
//    let conn = Connection::connect(&addr, ConnectionProperties::default())
//        .wait()
//        .expect("connection error");
//
//    info!("CONNECTED");
//
//    let channel_a = conn.create_channel().wait().expect("create_channel");
//    ////let channel_b = conn.create_channel().wait().expect("create_channel");
//
//    //let queue = channel_a
//    //    .queue_declare(
//    //        "hello",
//    //        QueueDeclareOptions::default(),
//    //        FieldTable::default(),
//    //    )
//    //    .wait()
//    //    .expect("queue_declare");
//
//    //info!("Declared queue {:?}", queue);
//
//    //info!("will consume");
//    //channel_b
//    //    .clone()
//    //    .basic_consume(
//    //        "hello",
//    //        "my_consumer",
//    //        BasicConsumeOptions::default(),
//    //        FieldTable::default(),
//    //    )
//    //    .wait()
//    //    .expect("basic_consume")
//    //    .set_delegate(Box::new(Subscriber {
//    //        channel: channel_b.clone(),
//    //    }));
//
//    let payload = b"Hello world!";
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
//
//    //loop {
//    //    let confirm = channel_a
//    //        .basic_publish(
//    //            "",
//    //            "hello",
//    //            BasicPublishOptions::default(),
//    //            payload.to_vec(),
//    //            BasicProperties::default(),
//    //        )
//    //        .wait()
//    //        .expect("basic_publish")
//    //        .wait()
//    //        .expect("publisher-confirms");
//    //    assert_eq!(confirm, Confirmation::NotRequested);
//    //}
//}
//
//pub fn listener () {
//    let mut executor = LocalPool::new();
//        let addr = std::env::var("AMQP_ADDR")
//            .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
//
//    executor.run_until(async {
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
//                "hello",
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
//                "hello",
//                "my_consumer",
//                BasicConsumeOptions::default(),
//                FieldTable::default(),
//            )
//            .await
//            .expect("basic_consume");
//        info!("[{}] state: {:?}", line!(), conn.status().state());
//
//        for delivery in consumer {
//            info!("received message: {:?}", delivery);
//            if let Ok(delivery) = delivery {
//                channel
//                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
//                    .await
//                    .expect("basic_ack");
//            }
//        }
//	info!("does it reach end");
//    })
//}
