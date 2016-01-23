use std::thread;
use std::time::Duration;
fn main() {
    let mut health = 12;
    for i in 2 .. 5 {
        thread::spawn(move || {
            health *= i;
        });
    }
    thread::sleep(Duration::from_millis(50));
    println!("{}", health); // 12
}
