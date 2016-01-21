mod game1 {
    // all of the module's code items go in here
    fn func1() {
        println!("Am I visible?");
    }
    
    pub fn func2() {
        println!("You called func2 in game1!");
    }
    
    pub mod subgame1 {
        pub fn subfunc1() {
            println!("You called subfunc1 in subgame1!");
        }
    }
    
    pub struct Magician {
        pub name : String,
        pub age: i32,
        power: i32
    }
}

fn main() {
    // game1::func1(); // <- error!
    game1::func2();
    // other code
    game1::subgame1::subfunc1();
    
    // let mag1 = game1::Magician { name: "Gandalf".to_string(), age: 725, power: 98 }; // error: field `power` of struct `game1::Magician` is private
}
