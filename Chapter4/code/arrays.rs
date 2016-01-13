fn main() {
  let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
  println!("{:?}", aliens);

  let zuxus = ["Zuxu"; 3];

  let mut empty: [i32; 0] = [];
  println!("{:?}", empty); // []

  println!("The first item is: {}", aliens[0]);

  println!("The third item is: {}", aliens[2]);

  let pa = &aliens;
  println!("The third item via pointer: {}", pa[2]);

  for ix in 0..aliens.len() {
      println!("Alien no {} is {}", ix, aliens[ix]);
  }

  for a in aliens.iter() {
      println!("The next alien is {}", a);
  }
}
