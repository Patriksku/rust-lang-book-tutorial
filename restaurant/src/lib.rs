// By default, a child module (and everything inside of it), is private from the perspective of the parent module.
// But Child modules can see anything that is defined inside of parent modules.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payments() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super used here as serve_order() is outside of this module.
        // super references the parent module, which in this case is crate (root)
        super::serve_order();
    }

    fn cook_order() {}
}

// Privacy rules for structs
// Structs, fields within the Structs, and their implementations are private by default, as well.
mod bed_and_breakfast {
    // If a struct contains at least one private field, we cannot create the struct directly from outside of this module!
    // In this case, we would have to use the associated function "summer".
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_bnb() {
    // Worthy of note, we can't create a Breakfast struct directly, as it has at least one private field (seasonal_fruit).
    // So we need to use the associated method to do so.
    let mut meal = bed_and_breakfast::Breakfast::summer("bread");
    meal.toast = String::from("Wheat");
}

// Privacy rules for enums
// To make an enum public, we only have to specify the enum itself as public.
// We do not have to explicitly specify "pub" for enum fields to access them (as opposed to structs).
// Because they simply would not be very useful if this was the case :)
mod heavens_kitchen {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_heavens_kitchen() {
    let order1 = heavens_kitchen::Appetizer::Soup;
    let order2 = heavens_kitchen::Appetizer::Salad;
}

// Module defined here, but get contents from different file with the same name as module here.
mod server;

// Use Keyword for scope.
//
// We have now brought server to scope!
// So we do not have to specify the full path any longer, more pretty code.
use crate::server::hosting;
// Above is full path. We can do the same but from relative path instead:
//
// use self::server::hosting; <-------------
//
// 'self' references the current module.
//
// The idiomatic way to bring functions to scope in Rust, is to bring the functions parent into scope.
// Makes the code clearer, and you also understand easier that the function being called is not a local function.
//
// The idiomatic way to bring enums, structs, or other items into scope, you should specify the full path.
//
// If we want external code to be able to access hosting (called 're-exporting'), we would declare the use statement public, like so:
//
// pub use crate::server::hosting;

pub fn allocate_server() {
    // server::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// The Use keyword is also used to bring external items into scope.
// Here we bring in the Rng trait from 'rand' dependency into scope.
use rand::CryptoRng;
use rand::Rng;

// We can use nested paths so we do not have to specify the full paths each time, if the items we bring in
// share a common parent in their tree:
//
// use rand::{CryptoRng, Rng};
//
// Another example, from:
//
// use std::io;
// use std::io::Write;
//
// to:
//
// use std::io::{self, Write};
//
// If we want all public items under 'io', we can do:
//
// use std::io::*;
