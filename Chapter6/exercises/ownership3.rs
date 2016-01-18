struct Alien {
    planet: String,
    n_tentacles: u32
}

fn main() {
    let mut klaatu = Alien { planet: "Venus".to_string(), n_tentacles: 15};

    // let kl2 = &klaatu;
    // kl2.n_tentacles = 10; // error
    // klaatu.n_tentacles = 12; // error

    // let klaatuc = klaatu;
    // let kl2 = &klaatu; // error
}
