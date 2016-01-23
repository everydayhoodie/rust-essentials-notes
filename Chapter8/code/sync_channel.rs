use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Duration;
type TokenType = i32;
struct Msg {
    typ: TokenType,
    val: String,
}

fn main() {
    let (tx, rx) = sync_channel(1); // buffer size 1
    tx.send(Msg { typ: 42, val: "Rust is cool".to_string() }).unwrap();
    println!("message 1 is sent");
    thread::spawn(move || {
        tx.send(Msg { typ: 43, val: "Rust is still cool".to_string() }).unwrap();
        println!("message 2 is sent");
    });
    println!("Waiting for 3 seconds...");
    thread::sleep(Duration::from_millis(3000));
    if let Some(msg) = rx.recv().ok() {
        println!("received message of type {} and val {}", msg.typ, msg.val);
    }
}
