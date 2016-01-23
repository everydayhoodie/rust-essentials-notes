extern crate num_cpus;
extern crate threadpool;

use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

fn main() {
    let ncpus = num_cpus::get();
    println!("The number of cpus in this machine is: {}", ncpus);
    
    let pool = ThreadPool::new(ncpus);
    for i in 0 .. ncpus {
        pool.execute(move || {
            println!("this is thread number {}", i);
        });
    }
    thread::sleep(Duration::from_millis(50));
}
