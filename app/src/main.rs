use std::fmt::Display;

// Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // The Static Lifetime
    let s: &'static str = "I have a static lifetime.";

    let mut result;
    result = longest(string1.as_str(), string2);

    println!("[1] the longest string is: {}", result);
    
    // error code, not compile
    // {
    //     let string3 = String::from("long string is long");
    //     result = longest(string1.as_str(), string3.as_str());
    // }
    // println!("[2] the longest string is: {}", result);

    let novel = String::from("Call me Ishael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// lifetime annotation 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str, 
    y: &'a str, 
    ann: T,
) -> &'a str 
where
    T:Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
