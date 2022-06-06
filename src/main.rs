fn main() {
  println!("Hello world");

  another_function(8, 'm');
}

fn another_function(x: i8, label: char) {
  println!("x = {}({})", x, label);
}