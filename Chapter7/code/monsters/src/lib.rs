pub fn print_from_monsters() {
    println!("Printing from crate monsters!");
}

#[derive(Debug)]
pub struct Zombi {
    pub health: i32,
    pub damage: i32
}

pub trait Monster {
    fn attack(&self);
    fn noise(&self) -> &'static str;
}

impl Monster for Zombi {
    fn attack(&self) {
        println!("I bite you! Your health lowers with {} damage points.", 2 * self.damage);
    }

    fn noise(&self) -> &'static str {
        "Aaargh!"
    }
}
