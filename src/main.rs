fn main() {
  let s1 = String::from("hello");
  
  // let s2 = s1;
  
  let s2 = s1.clone();
  
  println!("s1 = {}, s2 = {}", s1, s2);

  let s = String::from("My first string in rust");
  let w1 = first_word(&s);
  // s.clear();
  println!("word #1 = {}", w1);

  // array slice 
  let a = [1, 2, 3, 4, 5, 6, 7, 8];
  let slice = &a[1..3];
  assert_eq!(slice, &[2,3]);


  
}

// find out first word in line ard return 
// string slice
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}
