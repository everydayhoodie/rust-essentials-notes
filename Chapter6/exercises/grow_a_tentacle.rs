struct Alien {
    planet: String,
    n_tentacles: u32
}

fn main() {
    let mut klaatu = Alien { planet: "Venus".to_string(), n_tentacles: 15};
    println!("{}", klaatu.n_tentacles); // 15
    grow_a_tentacle(&mut klaatu);
    println!("{}", klaatu.n_tentacles); // 16
}

fn grow_a_tentacle(al: &mut Alien) {
    al.n_tentacles += 1;
}
