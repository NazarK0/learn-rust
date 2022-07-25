#![recursion_limit = "256"]

use json_macro::{json, Json};



fn main() {
    let width = 4.0;

    let desc = json!({
        "width": width,
        "height": (width * 9.0 / 4.0)
    });

    println!("{desc:#?}");
}
