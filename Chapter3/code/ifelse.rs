fn main() {
    let dead = false;
    let health = 48;
    if dead {
        println!("Game over!");
        return;
    }
    if dead {
        println!("Game over!");
        return;
    } else {
        println!("You still have a chance to win!");
    }
    if health >= 50 {
        print!("Continue to fight!");
    } else if health >= 20 {
        println!("Stop the battle and gain strength!");
    } else {
        println!("Hide and try to recover!");
    }
}
