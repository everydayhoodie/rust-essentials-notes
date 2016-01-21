struct Alien {
    planet: String,
    n_tentacles: u32
}

fn main() {
    let mut klaatu = Alien { planet: "Venus".to_string(), n_tentacles: 15};
    // let kl2 = klaatu;
    // println!("{}", klaatu.planet); // error

    let kl2 = &mut klaatu;
    kl2.n_tentacles = 14;
    println!("{} - {}", kl2.planet, kl2.n_tentacles);

    // klaatu.planet = "Pluto".to_string(); // error
    // println!("{} - {}", klaatu.planet, klaatu.n_tentacles); // error
}