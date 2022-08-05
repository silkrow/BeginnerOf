enum IpAddr {
	V4(String),
	V6(String),
}

enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		println!("......");
	}
}

fn main() {
	let _home = IpAddr::V4(String::from("127.0.0.1"));
	let _loopback = IpAddr::V6(String::from("::1"));

	let m = Message::Write(String::from("hello"));
	m.call();
	let _q = Message::Quit;

	let some_number = Some(5);
	let absent_number: Option<i32> = None;

	let a = Coin::Penny;
	println!("{}", in_cents(a));

	let b = Coin::Quarter(US::Alaska);
	in_cents(b);

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	let some_u8 = 0u8;
	match some_u8 {
		1 => println!("one"),
		3 => println!("three"),
		5 => println!("five"),
		7 => println!("seven"),
		_ => (),  // () is the unit value.
	}

	// Compare the following two implementations to have a taste of "if let" technique.
	let some_8 = Some(0u8);
	match some_8 {
		Some(3) => println!("three"),
		_ => (),
	}

	if let Some(3) = some_8 {
		println!("three");
	}

	// Another pair of example.
	let coin = Coin::Quarter(US::Alaska);
	let mut count = 0;
	match coin {
		Coin::Quarter(state) => println!("State quarter from {:?}!", state),
		_ => count += 1,
	}

	let coin1 = Coin::Quarter(US::Alaska);
	if let Coin::Quarter(state) = coin1 {
		println!("State quarter from {:?}!", state);
	} else {
		count += 1;
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i+1),
	}
}

#[derive(Debug)]
enum US {
	Alabama,
	Alaska,
}

enum Coin {
	Penny, 
	Nickel, 
	Dime, 
	Quarter(US),
}

fn in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Lucky Penny!");
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			25
		},
	}
}

