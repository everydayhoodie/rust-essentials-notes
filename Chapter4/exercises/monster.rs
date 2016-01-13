struct Monster {
    health: i32,
    damage: i32
}

fn main() {
    let m = Monster {health: 32, damage: 18};
    println!("{}", m.health);
    println!("{}", m.damage);
}
