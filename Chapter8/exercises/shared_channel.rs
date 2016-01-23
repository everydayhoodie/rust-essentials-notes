use std::sync::mpsc::channel;
use std::thread;

static NTHREADS: usize = 7;

fn main() {
    let (tx, rx) = channel();
    for i in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
                thread_tx.send(i).unwrap();
        });
    }

    // while let Some(msg) = rx.recv().ok() {
    //     println!("{}", msg);
    // }

    for _ in 0..NTHREADS {
        if let Some(msg) = rx.recv().ok() {
             println!("{}", msg);
         }
    }
}
