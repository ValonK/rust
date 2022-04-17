// give the mod a new name with the [as] keyword
use crate::front_of_house::hosting as Hosting;

// bring absolute path into scope
use crate::front_of_house::hosting;

//
//  bring relative path into scope use self::front_of_house::hosting;

use back_of_house::Appetizer;

mod front_of_house{
    pub mod hosting{
        pub fn add_two_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    // Absolute path
    hosting::add_two_waitlist();

    // Relative Path
    hosting::add_two_waitlist();
}

fn serve_order(){}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer{
        Soup, 
        Salad,
    }

    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
    
    fn cook_order(){}
}