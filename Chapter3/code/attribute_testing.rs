fn main() {
  println!("No tests are compiled, compile with rustc --test! ");
}

#[test]
fn arithmetic() {
  if 2 + 3 == 5 {
    println!("You can calculate!");
  }
}
