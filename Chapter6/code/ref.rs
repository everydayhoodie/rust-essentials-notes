fn main() {
    let n = 42;
    match n {
        ref r => println!("Got a reference to {}", r)
    }
    let mut m = 42;
    match m {
        ref mut mr => {
            println!("Got a mutable reference to {}", mr);
            *mr = 43;
        }
    }
    println!("m has changed to {}", m);
    
    let mag = Magician { name: "Gandalf", power: 4625};
    let name = {
        let Magician { name: ref ref_to_name, power: _} = mag;
        *ref_to_name
    };
    println!("The magician's name is {}", name);
}

struct Magician {
    name: &'static str,
    power: i32
}
