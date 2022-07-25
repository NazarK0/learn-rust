#![recursion_limit = "256"]

use json_macro::{json, Json};



fn main() {
    let fields = "Fields, W.C.";

    let desc = json!({
        "name": "John Smith",
        "actor": fields
    });

    println!("{desc:#?}");
}
