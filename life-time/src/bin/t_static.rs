use anyhow::Result;
use std::{error::Error, thread};

struct Container<T: 'static>(T);

struct Vec2 {
	x: i32,
	y: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
	let i = 32;
	let join_handle = thread::spawn(move || {
		println!("{i}");
	});
	join_handle.join().unwrap();
	let x = vec![1, 2, 3, 4, 5, 6, 7, 8].iter().map(|x| x + 3).sum::<i32>();
	let Vec2 { x, y } = Vec2 { x: 32, y: 42 };
	println!("{x} {y}");

	let x = 33;
	let n = Number{odd: false, value: 12, value3: &x};
	println!{"{n:?}"}

	use std::io::{self, Write};

  let a = b"Hello, there!\n";
	io::stdout().lock().write_all(b"Hello there\n").unwrap();

	Ok(())
}

#[derive(Clone, Copy, Debug)]
struct Number<'a> {
	odd: bool,
	value: i32,
	value3: &'a i32,
}

impl<'a> Number<'a> {
	fn is_strictly_positive(self) -> bool {
		self.value > 0
	}
}

fn print_number(n: Number) {
	match n {
		Number { value: 1, .. } => println!("One"),
		Number { value: 2, .. } => println!("Two"),
		Number { value, .. } => println!("Other {value}"),
	}
}
