use std::{cmp::Ordering, array}; // nested paths
use std::io::{self, Write};
use std::collections::*; //get all public items

mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house;


mod customer;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


fn deliver_order() {}
