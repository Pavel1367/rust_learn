// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }
//
// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }
//
// pub fn eat_at_restaurant2() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
// }

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }
//
// pub fn eat_at_restaurant2() {
//     let order1 = back_of_house::Appetizer::Salad;
//     let order2 = back_of_house::Appetizer::Soup;
// }
// use rand;
// use std::io::*;
//
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
// pub use crate::front_of_house::hosting;
//
// pub fn eat_at_restaurant() {
//     let secret_number = rand::random_range(0..101);
//     hosting::add_to_waitlist();
// }

// use std::fmt;
// use std::io::Result as IoResult;
//
// fn num_1() -> fmt::Result {}
// fn num_1() -> IoResult<()> {}

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
