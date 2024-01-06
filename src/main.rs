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
    println!("Controller mode!")
}

fn run_node() {
    println!("Node mode!")
}