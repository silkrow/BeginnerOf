fn main() {
	let mut s = String::from("hello");

	s.push_str(", world!");

    println!("{}", s);

	let _x = 5;
	let _y = _x;

	let s1 = String::from("hello");
	let s2 = s1.clone();

	println!("{}, world!", s1);
	println!("{}, world!", s2);

	let s = String::from("hello");

	let s = moves_ownership(s);
	let s = s;

	takes_ownership(s); // s's value moves into the function ... and so is no longer valid here.

	println!("{}, world!", s1);
	let s1 = String::from("hello");
	
	let (s, len) = calculate_length(s1);

	println!("The length of '{}' is {}.", s, len);

	let len = cal_length(&s);
	println!("The length of '{}' is {}.", s, len);
}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn moves_ownership(some_string: String) -> String {
	some_string
}

fn calculate_length(s: String) -> (String, usize){
	let length = s.len();

	(s, length)
}

fn cal_length(s: &String) -> usize {
	s.len()
}
