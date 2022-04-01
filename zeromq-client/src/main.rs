fn main() {
    let context = zmq::Context::new();

    // socket to receive messages on
    let receiver = context.socket(zmq::PULL).unwrap();
    receiver.connect("tcp://127.0.0.1:7000").unwrap();

    loop {
        let string = receiver.recv_string(0).unwrap().unwrap();

        // Show progress
        println!("{string}");
    }
}