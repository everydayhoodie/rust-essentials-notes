fn main() {
  let n = -1;
  println!("{} => {}", n, abs(n));
}

fn abs(x: i32) -> i32 {
  if x < 0 {
    -x
  } else {
    x
  }
}
