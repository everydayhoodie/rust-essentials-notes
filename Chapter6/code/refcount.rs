use std::rc::Rc;

#[derive(Debug)]
struct Alien {
    name: String,
    n_tentacles: u8
}

#[derive(Debug)]
struct Tentacle {
    poison: u8,
    owner: Rc<Alien>
}

fn main() {
    let dhark = Alien { name: "Dharkalen".to_string(), n_tentacles: 7 };
    let dhark_master = Rc::new(dhark);
    for i in 1u8 .. dhark_master.n_tentacles {
        let t = Tentacle { poison: i * 3, owner: dhark_master.clone() };
        println!("{:?}", t);
    }
}
