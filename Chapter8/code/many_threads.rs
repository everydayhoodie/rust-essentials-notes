use std::thread;
static NTHREADS: i32 = 10000;

fn main() {
    for i in 0 .. NTHREADS {
        let _ = thread::spawn(move || {
            println!("this is thread number {}", i);
        });
    }
}
