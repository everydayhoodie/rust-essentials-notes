struct Alien {
    planet: String,
    n_tentacles: u32
}

fn main() {
    let mut a1 = Box::new(Alien { planet: "Mars".to_string(), n_tentacles: 4 });
    println!("{}", a1.n_tentacles); // 4

    let mut a2 = &mut a1;
    println!("{}", a2.planet); // Mars
    a2.n_tentacles = 5;
    // println!("{}", a1.n_tentacles); // error
    // a1.planet = "Pluto".to_string();

    let n = Box::new(42);
    // *n = 67; // error

    let q = &*n;
    println!("{}", q); // 42
}
