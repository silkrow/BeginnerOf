fn main() {
	let mut counter = 0;

	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter*2;
		}
	};

	let mut number = 3;

	while number != 0 {
		println!("{}!", number);

		number = number - 1;
	}

	println!("The result is {}", result);

	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("the value is: {}", a[index]);

		index += 1;
	}

	for element in a.iter() {
		println!("the value is: {}", element);
	}

	println!("The 10th Fibonacci number is: {}", fib(10));
}

fn fib(x: i32) -> i32 {
	if x <= 0 {
		println!("Error!");
		return 0
	} else if x == 1 {
		return 0
	} else if x == 2 {
		return 1
	} else {
		return fib(x-1) + fib(x-2)
	}

}
