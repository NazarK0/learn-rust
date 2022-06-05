fn main() {
  let mut x = 5;
  println!("x= {}", x);
  x = 6;
  println!("x= {}", x);

  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("3 hours= {} in secons", THREE_HOURS_IN_SECONDS);

  let y = 18;
  let y = y + 1;

  {
    let y = y * 2;
    println!("inner y = {}", y);
  }

  println!("y = {}", y);
}