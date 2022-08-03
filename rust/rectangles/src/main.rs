#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

fn main() {
	let width1 = 30;
	let height1 = 50;

	println!(
		"The area of the rectangle is {} square pixels.", 
		area(width1, height1)
	);

	let rect2 = (40, 50);

	println!(
		"The area of the rectangle is {} square pixels.", 
		area2(rect2)
	);

	let rect3 = Rectangle { width: 50, height: 50 };

	println!(
		"The area of the rectangle is {} square pixels.",
		area3(&rect3)
	);

	println!("Print out the rectangle {:?}", rect3);
	println!("Print out the rectangle in pretty formatting {:#?}", rect3);

	println!(
		"The area of the rectangle is {} square pixels.", 
		rect3.area()
	);
	
	let rect4 = Rectangle { width: 30, height: 50 };
	let rect5 = Rectangle { width: 10, height: 40 };
	let rect6 = Rectangle { width: 60, height: 45 };

	println!("Can rect4 hold rect5? {}", rect4.can_hold(&rect5));
	println!("Can rect4 hold rect6? {}", rect4.can_hold(&rect6));
}
fn area(width: u32, height: u32) -> u32 {
	width * height
}

fn area2(r: (u32, u32)) -> u32 {
	r.0 * r.1
}

fn area3(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}
