struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x (&self) -> &T {
		&self.x
	}
}

impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

fn main() {
	let p1 = Point { x: 5, y: 10 };

	println!("p1.x = {}, p1.x = {}", p1.x(), p1.x);

	let p2 = Point { x: 1.0, y: 1.0 };

	println!("p2 has a distance from origin being {}", p2.distance_from_origin());

	// Two examples of using Option<T> enum, which will be performed with monomorphization during compilation to identify first one having i32 and the second one having f64
	let _int = Some(5);
	let _float = Some(5.0);
}
