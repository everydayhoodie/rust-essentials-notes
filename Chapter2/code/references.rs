fn main() {
    let health = 32;
    let mut game = "Space Invaders";

    println!("address of health-value: {:p}", &health); // prints 0x7fff51a1da14
    println!("address of game-value: {:p}", &game); // prints 0x7fff51a1da00

    println!("game-value: {}", game); // prints "Space Invaders"
}
