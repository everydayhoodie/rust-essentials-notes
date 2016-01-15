fn main() {
    let mut strength = 26;
    println!("My trpled strength equals {}", triples(strength)); // 78
    println!("My strength is still {}", strength); // 26

    strength = triples(strength);
    println!("My strength is now {}", strength); // 78

    strength = again(triples, strength);
    println!("I got some lucky to turn my strength into {}", strength); // 702 (= 3 * 3 * 78)

    strength = 78;
    let triples = |n| { 3 * n };

    strength = again(triples, strength);
    println!("My strength is now {}", strength); // 702

    strength = 78;
    strength = again(|n| { 3 * n }, strength);
    // strength = again(|n| 3 * n, strength);
    println!("My strength is now {}", strength); // 702

    let x: i32 = 42;
    let print_add = |s| {
        println!("x is {}", x);
        x + s
    };
    let res = print_add(strength);
    // here the closure is called and "x is 42" is printed
    assert_eq!(res, 744); // 42 + 702

    let m: i32 = 42;
    let print_add_move = move |s| {
        println!("m is {}", m);
        m + s
    };
    let res = print_add_move(strength); // strength == 702
    assert_eq!(res, 744); // 42 + 702
}

fn triples(s: i32) -> i32 { 3 * s }

fn again<F: Fn(i32) -> i32>(f: F, s: i32) -> i32 { f(f(s)) }
