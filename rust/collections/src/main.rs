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

	let _row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];
}
