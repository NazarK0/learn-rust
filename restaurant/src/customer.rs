use crate::back_of_house::Appetizer;
use std::fmt::Result;
use std::io::Result as IoResult; // alias

pub fn eat_at_restaurant() {
  let mut meal = crate::back_of_house::Breakfast::summer("Rye");

  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  let order1 = Appetizer::Soup;
  let order2 = crate::back_of_house::Appetizer::Salad;
}

// fn function1() -> Result {
//     // body
// }

// fn function2() -> IoResult<()> {
//     // body
// }
