fn main() {
    let n = 42i32;
    let m = &n;

    println!("The address of n is {:p}", m);
    println!("The value of n is {}", *m);
    println!("The value of n is {}", m);

    let q = &42;
    println!("{}", square(q)); // 1764

    let o = &n;
    println!("The address of o is {:p}", o);
    println!("The value of o is {}", *o);

    let mut u = 3.14f64;
    let v = &mut u;
    *v = 3.15;
    println!("The value of u is now {}", *v);
}

fn square(k: &i32) -> i32 {
    *k * *k
}
