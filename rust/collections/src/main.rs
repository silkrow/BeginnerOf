enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Text(String),
}

fn main() {
	let mut v1: Vec<i32> = Vec::new();

	let mut v2 = vec![1, 2, 3];

	v1.push(4);
	v2.push(4);

	let mut v3 = Vec::new();

	// v3.push(2147483747); // This won't work, cause Rust uses i32 by default, and 2147483747 overflows(it won't turn to u32 or i64 automatically)
	v3.push(1);

	let mut v4 = vec![1, 2, 3, 4, 5];

	let third: &i32 = &mut v4[2];
	println!("The third element is {}", third);

	match v4.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}

	v4.push(6);

	let mut v5 = vec![100, 32, 57];
	for i in &mut v5 {
		*i += 50;
		println!("{}", i);
	}	
	for i in &mut v5 {
		*i += 50;
		println!("{}", i);
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

	for i in &row {
		match i{
			SpreadsheetCell::Int(t) => println!("{}", t),
			SpreadsheetCell::Text(s) => println!("{}", s),
			_ => (),
		}
	}

	let mut _s = String::new();
	let data = "initial contents";

	let _sdata = data.to_string();

	let _hello = String::from("Dobrý den");
	let _hello = String::from("Hello");
	let _hello = String::from("नमस्ते");
	let _hello = String::from("こんにちは"); 
	let _hello = String::from("안녕하세요"); 
	let _hello = String::from("你好");
	let _hello = String::from("Olá");
	let _hello = String::from("Здравствуйте"); 
	let _hello = String::from("Hola");
	let _hello = String::from("Hallo");

	let s1 = String::from("hello ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // s1 is moved

	println!("{}", s2);
	println!("{}", s3);

	let mut s4 = String::from("foo");
	s4.push_str("bar");

	// add method in String has the following signature
	// fn add(self, s: &str) -> String {
	//	}
	// That's why s3 = s1 + &s2; is the common way to use +

	let s1 = String::from("tic");	
	let s2 = String::from("tac");	
	let s3 = String::from("toe");	

	println!("{}", s1 + "-" + &s2 + "-" + &s3);
	println!("{}", format!("{}-{}", s2, s3));

	let len = String::from("नमसत्").len();
	println!("{} has length {}!", "नमसत्", len); // 15

	println!("In chars:");
	for c in "नमसत्".chars() {
		println!("{}", c);
	}

	println!("In bytes:");
	for b in "नमसत्".bytes() {
		println!("{}", b);
	}

	use std::collections::HashMap;
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];

	let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

	let team_name = String::from("Blue");
	let score = scores.get(&team_name);

	match score {
		Some(&sco) => println!("{}", sco),
		None => (),
	}

	for (key, value) in &scores {
		println!("{}, {}", key, value);
	}

	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);
	scores.entry(String::from("Green")).or_insert(50);

	println!("{:?}", scores);

	let text = "hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1; // Recall that or_insert() returns a mutable reference
	}

	println!("{:?}", map);
}
