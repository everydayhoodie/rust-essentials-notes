extern crate num;
use num::traits::Float;

fn main() {
    println!("The square root of {} is {:?}", 42.0f32, sqroot(42.0f32));
    println!("The square root of {} is {:?}", 42.0f64, sqroot(42.0f64));
    // println!("The square root of {} is {:?}", 42, sqroot(42)); // error

    println!("The square root of {} is {:?}", 42.0f32, sqroot2(42.0f32));
}

fn sqroot<T: Float>(r: T) -> Result<T, String> {
    if r < num::zero() {
        return Err("Number cannot be negative!".to_string());
    }

    Ok(Float::sqrt(r))
}

fn sqroot2<T>(r: T) -> Result<T, String> where T: Float {
    if r < num::zero() {
        return Err("Number cannot be negative!".to_string());
    }

    Ok(Float::sqrt(r))
}
