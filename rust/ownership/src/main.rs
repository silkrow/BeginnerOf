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
	
	let a = 1;

	println!("This is a reference example for int! {}", ref_int(&a));

	let mut s = String::from("Hey");

	change(&mut s);


	let mut s = String::from("Hey");
	let r1 = &mut s;

	println!("{}, {}", r1, r1);

	let len = cal_length(r1);
	println!("{}", len);

	println!("still exists {}", r1);
	
	let s = String::from("Hello happy teachers' day");
	println!("{}", first_word(&s));

	let s = String::from("hello world");

	let _hello = &s[0..5];
	let _world = &s[6..11];

	println!("{} {}", r1, s); // This is weird, s has already been redefined, yet it's reference r1 still holds the string 'Hey' as the original s.

	println!("The first word of string '{}' is '{}'", s, first_word2(&s));

	println!("{}", &s[..]);
	

	// Let's check this weird behaviour once again.
	let sss = String::from("Morning");
	let rrr = &sss;
	println!("{} {}", sss, rrr);
	let sss = String::from("Evening");
	println!("{} {}", sss, rrr);
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

fn ref_int(n: &u32) -> u32 {
	*n
}

fn change(s: &mut String) {
	s.push_str(", hello!!");
}

fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes(); // Convert the string to an array of bytes.

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}
	s.len()
}

fn first_word2(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}
