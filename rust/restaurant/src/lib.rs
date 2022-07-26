mod front_of_house;

pub use crate::front_of_house::hosting; // Added the module into crate root with a absolute path

// use self::front_of_house::hosting; // Added the module into crate root with a relative path, note the 'self' keyword at the beginning

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();

	// With the help of 'use' keyword
	hosting::add_to_waitlist();

	// Order a breakfast in the summer with Rye toast
	let mut meal = back_of_house::Breakfast::summer("Rye");
	// Change our mind about what bread we'd like
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please", meal.toast);

	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
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

	pub enum Appetizer {
		Soup,
		Salad,
	}
}
