#![feature(asm)]
fn substract(a: i32, b: i32) -> i32 {
    let sub: i32;
    unsafe {
        asm!("sub $2, $1; mov $1, $0"
            : "=r"(sub)
            : "r"(a), "r"(b)
        );
    }
    sub
}

fn main() {
    println!("{}", substract(42, 7));
}
