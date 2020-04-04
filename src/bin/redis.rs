use orizuru::Consumer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Job {
    id: u64,
}

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();
    let worker = Consumer::new("consumer-1".into(), "orizuru-example".into(), con);

    println!("Starting consumer with queue `default`");

    while let Some(task) = worker.next::<Job>() {
        if task.is_err() {
            continue;
        }

        let task = task.unwrap();

        println!("Task: {:?}", task.payload());
    }
}
//
//
//
//use redis::{self, transaction, Commands};
//
//fn main() {
//
//    if let Err(err) = redis_subscribe() {
//	println!("{:?}", err);
//    };
//}
//fn redis_subscribe()  -> redis::RedisResult<()>{
//    //let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//    //let mut con = client.get_connection().unwrap();
//
//    //let () = con.set("key1", b"foo").await?;
//
//    //let result = redis::cmd("HGET").arg("messages").arg("1").query(&mut con);
//    //let result = redis::cmd("HGET")
//    //    .arg(&["messages", "1"])
//    //    .query_async(&mut con)
//    //    .await;
//    //dbg!(result.unwrap());
//    //dbg!(result.to_string());
//
//    //let result = redis::cmd("MGET")
//    //    .arg(&["key1", "key2"])
//    //    .query_async(&mut con)
//    //    .await;
//    //assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
//    //Ok(())
//    let client = redis::Client::open("redis://127.0.0.1:6379")?;
//    let mut con = client.get_connection()?;
//    let mut pubsub = con.as_pubsub();
//    pubsub.subscribe("chan4")?;
//    //pubsub.subscribe("channel_2")?;
//    println!("lmao 1");
//    loop {
//	println!("lmao 2");
//	let msg = pubsub.get_message()?;
//	println!("lmao 3");
//	let payload : String = msg.get_payload()?;
//	println!("lmao 4");
//	println!("channel '{}': {}", msg.get_channel_name(), payload);
//    }
//    println!("lmao end");
//}
