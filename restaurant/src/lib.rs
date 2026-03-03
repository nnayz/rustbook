/*
 * Crate root
 *
 * Paths: Absolute(starts from the crate name) and Relative(
 * starts from current modile and uses self, super or an identifier in the current module)
 */

/*
 * The module tree should be defined in the lib.rs file.
 */

/*
 * Making structs and enums public
 */

/*
 * Nested paths
 *
 * use std::{cmp::Ordering, io};
 * use std::io::{self, Write};
 * use std::collections::*; # `glob operation`
 */

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/*
 * relative paths with super
 */

fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    #[derive(Debug)]
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

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    crate::back_of_house::fix_incorrect_order();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let meal2 = Breakfast::summer("Rye");

    println!("{:#?}", meal2);
}
