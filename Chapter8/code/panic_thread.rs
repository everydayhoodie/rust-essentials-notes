use std::thread;
fn main() {
    let result = thread::spawn(move || {
        panic!("I have fallen into an unrecoverable trap!");
    }).join();
    if result.is_err() {
        println!("This child has panicked");
    }
}
