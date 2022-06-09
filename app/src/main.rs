use std::fs::File;
use std::io::{self, Read};

fn main() {
    let username = read_username_from_file().unwrap();
    println!("username: {}",username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
