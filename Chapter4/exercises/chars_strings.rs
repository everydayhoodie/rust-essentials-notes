fn main() {
    let magician = "Merlin";
    // println!("{}, {}", magician.to_string()[0], magician.to_string()[4]);  // compile error.

    let greeting = "Hello, 世界";

    for b in greeting.bytes() {
        println!("{}", b);
    }

    for c in greeting.chars() {
        println!("{}", c);
    }
}
