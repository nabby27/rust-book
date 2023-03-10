mod front_of_house;
mod back_of_house;

mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house::breakfast::Breakfast;
    
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
        
        // Relative path
        // front_of_house::hosting::add_to_waitlist(); // Invalid because eat_at_restaurant is child of customer
        
        // with use
        hosting::add_to_waitlist();

        let mut meal = Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        // meal.seasonal_fruit = String::from("blueberries"); // Invalid
        println!("I'd like {} toast please", meal.toast);

        let order1 = super::back_of_house::appetizer::Appetizer::Soup;
        let order2 = super::back_of_house::appetizer::Appetizer::Salad;
    }
}

fn deliver_order() {}
