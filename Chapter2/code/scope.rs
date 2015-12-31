fn main() {
    let outer = 42;

    { // start of code block
        let inner = 3.14;
        println!("block variable: {}", inner);
        let outer = 99; // shadows the first outer variable
        println!("block variable outer: {}", outer);
    } // end of code block

    println!("outer variable: {}", outer);
}
