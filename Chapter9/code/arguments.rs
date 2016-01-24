use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("The program's name is {}", args[0]);
    for arg in args.iter() {
        println!("Next argument is: {}", arg);
    }
    println!("I got {:?} arguments", args.len() - 1);
    for n in 1 .. args.len() {
        println!("The {}th argument is {}", n, args[n]);
    }
    let osvars = env::vars();
    for(key, value) in osvars {
        println!("{}, {}", key, value);
    }
}
