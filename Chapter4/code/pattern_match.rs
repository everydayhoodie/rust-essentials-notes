use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
        .ok()
        .expect("Failed to read number");
    let input_num: Result<u32, _> = buf.trim().parse();
    // println!("Unwrap found {}", input_num.unwrap());

    match input_num {
        Ok(num) => println!("{}", num),
        Err(ex) => println!("Please input an integer number! {}", ex)
    }

    let input_num: Result<u32, _> = buf.trim().parse();
    if let Ok(val) = input_num {
        println!("Matched {:?}!", val);
    } else {
        println!("No match!");
    }

    while let Ok(val) = input_num {
        println!("{:?}!", val);
        if val == 42 { break }
    }
}
