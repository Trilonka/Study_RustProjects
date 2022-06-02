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

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}
// ------
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        super::deliver_order();
    }

    fn cook_order() {}
}
// ------
mod back_of {
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

pub fn eat_at() {
    let mut meal = back_of::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
// ------
mod back {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat() {
    let order1 = back::Appetizer::Salad;
    let order2 = back::Appetizer::Soup; 
}
// ------
mod front_of_house_use {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod customer {
    use super::front_of_house_use::hosting;
    pub fn eat_at_restaurant_use() {
        hosting::add_to_waitlist();
    }
}
// ------ how to use similar Result from different modules
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// ------ using 'as' to set a new name for std::io::Result

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// ------