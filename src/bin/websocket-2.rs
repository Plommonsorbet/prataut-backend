/// An example of a chat web application server
extern crate ws;
//use futures::executor::LocalPool;
use prataut_backend::{Server};
//use prataut_backend::{listener, pub_sub, sender, Server};
use std::cell::Cell;
use std::rc::Rc;
use ws::{listen, Handler, Handshake, Message, Request, Response, Result, Sender};
//use std::{thread, time};

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
