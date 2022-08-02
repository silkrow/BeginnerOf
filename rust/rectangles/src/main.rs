struct Rectangle {
	width: u32,
	height: u32,
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
		"The area of the rectangle is {} square pixels,", 
		area2(rect2)
	);

	let rect3 = Rectangle { width: 50, height: 50 };

	println!(
		"The area of the rectangle is {} square pixels.",
		area3(&rect3)
	)
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
