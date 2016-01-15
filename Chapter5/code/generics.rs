fn main() {
    let magic_pair: Pair<i32> = Pair { first: 7, second: 42 };
    println!("{}", magic_pair.first); // 7

    let pair_of_magicians: Pair<&str> = Pair { first: "Gandalf", second: "Sauron" };
    println!("{}", pair_of_magicians.second); // Sauron

    let a = second(magic_pair);
    println!("{}", a); // 42

    let x :Option<i8> = Some(5);
    let pi: Option<f64> = Some(3.25259265359);
    let none: Option<f64> = None;
    let none2 = None::<f64>;
    let name: Option<&str> = Some("Joyce");
    // let magic: Option<f32> = Some(42); // error

    let p1 = Person{ name: "James Bond", id: 7};
    let p2 = Person{ name: "Vin Diesel", id: 12};
    let p3 = Person{ name: "Robin Hood", id: 42};

    let op1: Option<Person> = Some(p1);
    let pvec: Vec<Person> = vec![p2, p3];

    let m = sqroot(42.0);
    // let m = sqroot(-5.0);
    match m {
        Ok(sq) => println!("The square root of 42 is {}", sq),
        Err(str) => println!("{}", str),
    }
}

struct Pair<T> {
    first: T,
    second: T,
}

struct Person {
    name: &'static str,
    id: i32
}

fn second<T>(pair: Pair<T>) -> T {
    pair.second
}

fn sqroot(r: f32) -> Result<f32, String> {
    if r < 0.0 {
        return Err("Number cannot be negative!".to_string());
    }

    Ok(f32::sqrt(r))
}
