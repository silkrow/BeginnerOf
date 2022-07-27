fn main() {
	let number = 3;

	if number < 5 {
		println!("condition was true");
	} else {
		println!("condition was false");
	}

	let _x = if number > 3 {
		"a number larger than 3"
	} else {
		"a number smaller than 3"
	};
}
