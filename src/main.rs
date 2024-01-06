use std::env;

fn main() {
    let mode = env::var("MODE").unwrap_or_else(|_| "node".to_string());
    if mode == "controller" {
        run_controller();
    } else {
        run_node();
    }
}

fn run_controller() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher.bind("tcp://*:5555").unwrap();

    loop {
        publisher.send("Hello, Nodes!",0).unwrap();
    }
}

fn run_node() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5555").unwrap();
    subscriber.set_subscribe(b"").unwrap();

    loop {
        let message = subscriber.recv_string(0).unwrap().unwrap();
        println!("Received: {}", message);
    }
}