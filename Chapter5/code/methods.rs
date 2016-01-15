fn main() {
    let mut bork = Alien{ health: 100, damage: 5 };
    let mut berserk = Alien::new(150, 15);
    println!("{}", berserk.health); // 100
    println!("{}", Alien::warn());

    berserk.attack();
    println!("{}", berserk.health); // 100
    berserk.attack_and_suffer(31);
    println!("{}", berserk.health); // 69
}

struct Alien {
    health: u32,
    damage: u32
}

impl Alien {
    fn new(mut h: u32, d: u32) -> Alien {
        // constraints:
        if h > 100 {
            h = 100;
        }
        Alien { health: h, damage: d}
    }

    fn warn() -> &'static str {
        "Leave this planet imidiately or perish!"
    }

    fn attack(&self) {
        println!("I attack! Your health lowers with {} damage points.", self.damage);
    }

    // fn attack(&self) {
    //     self.health -= 10;
    // }

    fn attack_and_suffer(&mut self, damage_from_other: u32) {
        self.health -= damage_from_other;
    }
}
