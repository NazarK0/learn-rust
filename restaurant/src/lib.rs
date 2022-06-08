use std::{cmp::Ordering, array}; // nested paths
use std::io::{self, Write};
use std::collections::*; //get all public items

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


mod customer {
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
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


fn deliver_order() {}
