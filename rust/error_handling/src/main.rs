use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
// Complicated way.
	let f = File::open("hello.txt");
	let f = match f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {:?}", e),
			},
		other_error => panic!("Problem opening the file: {:?}", other_error),
		},
	};

// Simple way.
	let f = File::open("hello.txt").unwrap_or_else(|error| { // Method unwrap_or_else is in the standard library.
		if error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|error| {
				panic!("Problem creating the file: {:?}", error);
			})
		} else {
			panic!("Problem opening the flie: {:?}", error);
		}
	});
}

fn read_username_from_file() -> Result<String, io::Error> { // Return the username(the result of running read_to_string on a file. Any failure would lead to an io::Error)
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}
