fn verbose(x: i32) -> &'static str {
    let result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

fn main() {
    let n = 13;
    println!("verbose {}", verbose(n));
}
