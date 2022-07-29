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
}
