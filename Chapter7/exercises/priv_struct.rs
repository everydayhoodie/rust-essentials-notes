mod game1 {
    #[derive(Debug)]
    pub struct Magician {
        pub name : String,
        pub age: i32,
        power: i32
    }

    impl Magician {
        pub fn new(nm: String, ag: i32, pw: i32) -> Magician {
            Magician {name: nm, age: ag, power: pw}
        }
    }
}

fn main() {
    let mag1 = game1::Magician::new("Gandalf".to_string(), 725, 98);
    println!("I just made a magician {:?}", mag1);
}
