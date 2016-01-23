use std::thread;
use std::sync::{Arc, Mutex};
fn main() {
    let mut health = 12;
    println!("health before: {:?}", health);
    let data = Arc::new(Mutex::new(health));
    for i in 2 .. 5 {
        let mutex = data.clone();

        thread::spawn(move || {
            let health = mutex.lock();
            match health {
                Ok(mut health) => *health *= i,
                Err(str) => println!("{}", str)
            }
        }).join().unwrap();
    }
    health = *data.lock().unwrap();
    println!("health after: {:?}", health);
}
