use std::{fs, io, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let username = read_username_from_file().unwrap();
    println!("username: {}",username);

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
