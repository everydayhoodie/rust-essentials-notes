use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(move || {
        println!("Hello from the goblin in the spawned thread!");
    });
    // thread::sleep_ms(50); // Deprecated since 1.6.0: replaced by std::thread::sleep
    // thread::sleep(Duration::from_millis(50));
    // do other work in the meantime
    let output = handle.join().unwrap();
    println!("{:?}", output); // ()
    
    thread::spawn(move || {
        // work done in child thread
    }).join();
}
