/// An example of a chat web application server
extern crate ws;
use futures::executor::LocalPool;
use prataut_backend::{listener, pub_sub, sender};
use std::cell::Cell;
use std::rc::Rc;
use uuid::Uuid;
use
use ws::{listen, Handler, Handshake, Message, Request, Response, Result, Sender};

// This can be read from a file
static INDEX_HTML: &'static [u8] = br#"
<!DOCTYPE html>
<html>
	<head>
		<meta charset="utf-8">
	</head>
	<body>
      <pre id="messages"></pre>
			<form id="form">
				<input type="text" id="msg">
				<input type="submit" value="Send">
			</form>
      <script>
        var socket = new WebSocket("ws://" + window.location.host + "/ws");
        socket.onmessage = function (event) {
          var messages = document.getElementById("messages");
          messages.append(event.data + "\n");
        };
        var form = document.getElementById("form");
        form.addEventListener('submit', function (event) {
          event.preventDefault();
          var input = document.getElementById("msg");
          socket.send(input.value);
          input.value = "";
        });
		</script>
	</body>
</html>
    "#;

// Server web application handler
struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {
    //
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        // Using multiple handlers is better (see router example)
        let new_uuid = Uuid::new_v4();
        match req.resource() {
            // The default trait implementation
            "/ws" => Response::from_request(req),

            // Create a custom response
            "/" => Ok(Response::new(200, "OK", INDEX_HTML.to_vec())),

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // 3.
        self.count.set(self.count.get() + 1);
        let number_of_connection = self.count.get();

        if number_of_connection > 5 {
            // panic!("There are more user connection than expected.");
        }

        // 4.
        let open_message = format!(
            "{} entered and the number of live connections is {}",
            &handshake.peer_addr.unwrap(),
            &number_of_connection
        );
        // println!("{}", &handshake.local_addr.unwrap());

        println!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let addr = std::env::var("AMQP_ADDR")
            .unwrap_or_else(|_| "amqp://user:password@127.0.0.1:5672/%2f".into());

        //let mut executor = LocalPool::new();
        //pub_sub(&addr, "HIIIII").unwrap();
        dbg!(&msg);
        dbg!(self.out.token());
        dbg!(self.out.connection_id());
        // Broadcast to all connections
        self.out.broadcast(msg);
	self.out.send("this is a private message sent only to this client.");
        //dbg!(self.out.connection_id());
        //executor.run_until(sender(&addr, "This is my message from client."));
        //executor.run_until(sender("This is my message from client."));
        //let result = executor.run_until(listener(addr.to_string()));
        //dbg!(result);
        //self.out.send();
        Ok(())
        // Broadcast to all connections
        //self.out.broadcast(msg)
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // Listen on an address and call the closure for each connection
    listen("127.0.0.1:8000", |out| Server {
        out,
        count: Rc::new(Cell::new(0)),
    })
    .unwrap()
}
