
fn main() {
    let mut s = String::from("абвггдеєжфазїҐ");

    let updated = delete_duplicates(&s);

    println!("{}",updated);
}

fn delete_duplicates(source: &str) -> String {
    let mut chars_vec = Vec::with_capacity(source.chars().count());

    for ch in source.chars() {
        chars_vec.push(ch);
    }

    chars_vec.sort();
    chars_vec.dedup();

    let mut result_string = String::new();

    for ch in chars_vec.iter() {
        result_string.push(*ch);
    }

    result_string
}
