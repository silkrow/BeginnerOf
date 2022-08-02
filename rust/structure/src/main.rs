struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
	let mut user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername"),
		active: true,
		sign_in_count: 1,
	};

	user1.email = String::from("anotheremail@example.com");

	let user2 = User {
		email: String::from("some@example.com"),
		username: String::from("somename"),
		..user1
	};

	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

	println!("{}", black.1);
	let t:(u32, u32, u32) = (1, 2, 3);
	println!("{:?}", t); // But can't do this to tuple structure
}


fn build_user(email: String, username: String) -> User {
	User {
		email, // field init shorthand syntax. Only applied when field matches with the value's name.
		username,
		active: true,
		sign_in_count: 1,
	}
}
