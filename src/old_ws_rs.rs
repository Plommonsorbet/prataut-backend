
//#[derive(Clone, Debug)]
//struct Subscriber {
//    channel: Channel,
//}

//#[derive(Serialize, Deserialize, Debug)]
//#[serde(tag = "type")]
//enum ClientRequest {
//    Poll,
//    Send { message: String },
//}
//
//// Server web application handler
//pub struct Server {
//    pub out: Sender,
//}
////// Server web application handler
////pub struct Router {
//    //pub sender: Sender,
//    //pub inner: Box<Handler>,
//    ////pub count: Rc<Cell<u32>>,
////}
////pub struct NotFound;
////
////impl ws::Handler for NotFound {
//    //fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
//        //// This handler responds to all requests with a 404
//        //let mut res = ws::Response::from_request(req)?;
//        //res.set_status(404);
//        //res.set_reason("Not Found");
//        //Ok(res)
//    //}
////}
////
////impl Handler for Router {
//    //fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
//        //// Clone the sender so that we can move it into the child handler
//        //let out = self.sender.clone();
////
//        //match req.resource() {
//            //// Route to a data handler
//            //"/ws1" => self.inner = Box::new(Server { out: out }),
////
//            //// Route to another data handler
//            //"/ws1" => self.inner = Box::new(Server { out: out }),
////
//            //// Use a closure as the child handler
//            //"/closure" => {
//                //self.inner = Box::new(move |msg: ws::Message| {
//                    //println!("Got a message on a closure handler: {}", msg);
//                    //out.close_with_reason(ws::CloseCode::Error, "Not Implemented.")
//                //})
//            //}
////
//            //// Use the default child handler, NotFound
//            //_ => (),
//        //}
////
//        //// Delegate to the child handler
//        //self.inner.on_request(req)
//    //}
////
//    //// Pass through any other methods that should be delegated to the child.
//    ////
//    //// You could probably use a macro for this if you have many different
//    //// routers or were building some sort of routing framework.
////
//    //fn on_shutdown(&mut self) {
//        //self.inner.on_shutdown()
//    //}
////
//    //fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
//        //self.inner.on_open(shake)
//    //}
////
//    //fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
//        //self.inner.on_message(msg)
//    //}
////
//    //fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
//        //self.inner.on_close(code, reason)
//    //}
////
//    //fn on_error(&mut self, err: ws::Error) {
//        //self.inner.on_error(err);
//    //}
////}
//
//impl Handler for Server {
//    //dbg!(self.out.token());
//    //dbg!(self.out.token(),self.out.connection_id());
//    fn on_request(&mut self, req: &Request) -> ws::Result<(Response)> {
//        // Using multiple handlers is better (see router example)
//        let new_uuid = Uuid::new_v4();
//        match req.resource() {
//            // The default trait implementation
//            //"/ws" => Response::from_request(req),
//
//            // Create a custom response
//            //"/" => Ok(Response::new(200, "OK", fs::read("client.html").unwrap())),
//            _ => Response::from_request(req),
//            //_ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
//        }
//    }
//    fn on_open(&mut self, handshake: Handshake) -> ws::Result<()> {
//        //self.count.set(self.count.get() + 1);
//
//        //let number_of_connection = self.count.get();
//        info!("{:?}", handshake.request);
//
//        //"{} entered and the number of live connections is {}",
//        let open_message = format!(
//            "{} entered and the number of live connections is",
//            &handshake.peer_addr.unwrap(),
//            //&number_of_connection
//        );
//
//        println!("{}", &open_message);
//        self.out.broadcast(open_message);
//
//        Ok(())
//    }
//
//    // Handle messages recieved in the websocket (in this case, only on /ws)
//    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
//        info!("Received message {}", &msg);
//        if let Ok(text) = msg.as_text() {
//            let client_request: serde_json::Result<ClientRequest> = serde_json::from_str(text);
//            if let Ok(req) = client_request {
//                match req {
//                    ClientRequest::Poll => {
//                        self.out.send(Message::text("POLLING."));
//                    }
//                    ClientRequest::Send { message } => {
//                        info!(
//                            "SEND MSG! token: {:?}, conn_id: {}\nmsg: {}",
//                            self.out.token(),
//                            self.out.connection_id(),
//                            message
//                        );
//                        self.out.broadcast(Message::text(message));
//                    }
//                }
//            } else {
//                error!(
//                    "Message can't be converted to json! token: {:?}, conn_id: {}\nreq: {:?}",
//                    self.out.token(),
//                    self.out.connection_id(),
//                    client_request
//                );
//            }
//        } else {
//            error!(
//                "Message is not utf8! token: {:?}, conn_id: {}",
//                self.out.token(),
//                self.out.connection_id()
//            );
//        };
//        let sndr = self.out.clone();
//        thread::spawn(move || {
//            let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//            let mut con = client.get_connection().unwrap();
//            let mut pubsub = con.as_pubsub();
//            pubsub.subscribe("channel_1").unwrap();
//            pubsub.subscribe("channel_2").unwrap();
//
//            loop {
//                let msg = pubsub.get_message().unwrap();
//                let payload: String = msg.get_payload().unwrap();
//
//                sndr.send(Message::text(payload.to_string()));
//                println!("channel '{}': {}", msg.get_channel_name(), &payload);
//                sndr.send(Message::text(&payload));
//            }
//        });
//        //let chat_message: ChatMessage = serde_json::from_str(&msg.as_text()?)?;
//
//        info!("leaving send!");
//        Ok(())
//        //let addr = std::env::var("AMQP_ADDR")
//        //    .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
//
//        //let result = executor.run_until(listener(addr.to_string()));
//        //let addr =
//        //    std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
//
//        //Ok(())
//        //match &msg.as_text().unwrap().to_string() {
//        //    x if x.clone() == "i_wanna_listen".to_string() => {
//        //	dbg!("this be awesum");
//        //	for i in 1..15 {
//
//        //	    let ten_millis = time::Duration::from_millis(1000);
//        //	    let now = time::Instant::now();
//        //
//        //	    thread::sleep(ten_millis);
//        //	    self.out.send(format!("I: {}", i));
//        //	};
//
//        //    }
//        //    _ => ()
//        //};
//        //let chat_message: ChatMessage = serde_json::from_str(&msg.as_text().unwrap()).expect("not in json format");
//        //dbg!(&msg.as_text());
//        //dbg!(self.out.connection_id());
//
//        //let addr = std::env::var("AMQP_ADDR")
//        //    .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());
//        // Broadcast to all connections
//        //self.out.broadcast(msg);
//
//        //self.out.send("this is a private message sent only to this client.");
//        //for i in 1..100 {
//        //    self.out.send(format!("I: {}", i));
//        //};
//        //dbg!(self.out.connection_id());
//        //executor.run_until(sender(&addr, "This is my message from client."));
//        //executor.run_until(sender("This is my message from client."));
//        //let result = executor.run_until(listener(addr.to_string()));
//        //dbg!(result);
//        //self.out.send();
//        // Broadcast to all connections
//        //self.out.broadcast(msg)
//        //let mut executor = LocalPool::new();
//        //pub_sub(&addr, "HIIIII").unwrap();
//        //match msg {
//        //    Text { msg } => {
//        //    let chat_message: ChatMessage = serde_json::from_str(&msg).expect("not in json format");
//        //	dbg!(chat_message);
//        //    }
//        //    Binary { bin } => {
//        //	dbg!("in binary format");
//        //    }
//        //};
//    }
//}
