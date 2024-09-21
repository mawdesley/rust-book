mod front_of_house;
pub use crate::front_of_house::hosting;

fn deliver_order() {}
mod back_of_house {
    fn fix_oncorrect_order() {
        cook_order();
        super::deliver_order();
        // or
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {}

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

mod customer {
    use super::front_of_house::hosting;
    pub fn eat_at_restaurant () {
        // abs
        crate::front_of_house::hosting::add_to_waitlist();

        //rel
        super::front_of_house::hosting::add_to_waitlist();

        // use
        hosting::add_to_waitlist();

        let mut meal = super::back_of_house::Breakfast::summer("sourdough");

        meal.toast = String::from("rye");

        println!("I'd like {} toast please", meal.toast);

        // this is privado
        // meal.seasonal_fruit = String::from("blueberries");


    }
}