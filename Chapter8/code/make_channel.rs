use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
fn make_chan() -> Receiver<i32> {
    let (tx, rx) = channel();
    tx.send(7).unwrap();
    rx
}
fn main() {
    let rx = make_chan();
    if let Some(msg) = rx.recv().ok() {
        println!("received message {}", msg);
    }
}
