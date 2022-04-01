use std::thread;
use std::time::Duration;
use std::io::{self, BufRead};

fn main() {
    let context = zmq::Context::new();

    // Socket to send messages on
    let sender = context.socket(zmq::PUSH).unwrap();
    sender.bind("tcp://127.0.0.1:7000").unwrap();

    println!("Press Enter when the workers are ready: ");
    let stdin = io::stdin();
    stdin.lock().lines().next();

    // Send 100 tasks
    for q in 0..100 {
        let workload_str = format!("{}", q);
        thread::sleep(Duration::from_millis(1000));
        sender.send(&workload_str, 0).unwrap();
    }

    println!("!end");
}