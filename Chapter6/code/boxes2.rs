struct Alien {
    planet: String,
    n_tentacles: u32
}

struct Recurs {
    list: Vec<u8>,
    rec_list: Option<Box<Recurs>>
}

fn main() {
    let n = Box::new(42);
    let mut m = n;
    *m = 67;
    // println!("{}", n); // error: use of moved: value: `n`
    println!("{}", m); // 67

    let mut a1 = Box::new(Alien { planet: "Mars".to_string(), n_tentacles: 4 });
    let a2 = a1;
    println!("{}", a2.n_tentacles); // 4

    use_alien(a2);
    // println!("{}", a2.n_tentacles); // error: use of moved value: `a2.n_tentacles`
    // use_alien2(&*a2);
}

fn use_alien(a: Box<Alien>) {
    println!("An alien from planet {} is freed after the closing brace", a.planet);
}

fn use_alien2(a: &Alien) {
    println!("An alien from planet {} is freed after the closing brace", a.planet);
}
