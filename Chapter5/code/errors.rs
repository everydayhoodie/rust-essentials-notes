fn main() {
    let x = 3;
    let y = 0;
    x / y; // error
    
    if y == 0 {
         panic!("Dicision by 0 occurred, exiting");
    }
    println!("{}", div(x, y));
    
    assert!(x == 5); // thread '<main>' panicked at 'assertion failed: x == 5'
    assert!(x == 5, "x is not equal to 5!"); // thread '<main>' panicked at 'x is not equal to 5!'
    assert_eq!(x, 5); // thread '<main>' panicked at 'assertion failed: `(left == right)` (left: `3`, right: `5`)'
    unreachable!(); // thread '<main>' panicked at 'internal error: entered unreachable code'
}

fn div(x: i32, y: i32) -> f32 {
    (x / y) as f32
}
