fn main() {
    let magician1 = "Marlin";
    let greeting = "Hello, 世界!";

    let magician2: &'static str = "Gandalf";

    println!("Magician {} greets magician {} with {}", magician1, magician2, greeting);

    let mut str1 = String::new();
    let mut str2 = String::with_capacity(25);
    let mut str3 = magician1.to_string();

    let sl1 = &str3;

    let c1 = 'q';
    str1.push(c1);

    println!("{}", str1); // q
    str1.push_str("Level 1 is finished - ");
    println!("{}", str1); // q Level 1 is finished -
    str1.push_str("Rise up to Level 2");
    println!("{}", str1); // q Level 1 is finished - Rise up to Level 2

    for c in magician1.chars() {
        print!("{} - ", c);
    }

    for word in str1.split(" ") {
        print!("{} / ", word);
    }

    let str5 = str1.replace("Level", "Floor");

    println!("Length of str1: {}", how_long(&str1));
    println!("Length of str1: {}", how_long(&str1[..]));
}

fn how_long(s: &str) -> usize {
    s.len()
}
