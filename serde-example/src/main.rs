use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
	x: i32,
	y: i32,
}

fn main() {
	let point = Point { x: 3, y: 4 };
	let serialized: String = serde_json::to_string(&point).unwrap();
	println!("{}", serialized);

	let point: Point = serde_json::from_str(&serialized).unwrap();
	println!("{:?}", point);
	if true {
    println!("hello")
	}

	let mut s = String::from("hello");
	s.push_str(" str");
	println!("{}", s);

	println!("\u{e7a8}")
}
