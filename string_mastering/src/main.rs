fn main() {
    let s = String::from("Привіт!");

    print!("chars 1: ");
    for ch in s.chars() {
        print!("{}, ", ch);
    }
    print!("\n");
    print!("char sizes 1: ");
    for ch in s.chars() {
        print!("{}, ", ch);
    }
    print!("\n");
}
