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
}
