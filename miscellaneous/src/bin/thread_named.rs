use std::thread::{self};

use anyhow::Result;

fn main() -> Result<()> {
	let join = thread::Builder::new().name("test_hello".to_string()).spawn(move || {
		println!("hello world");
		println!("threa name: {}", thread::current().name().unwrap_or("unamed"));
	})?;
	join.join().expect("Couldn't join on the associated thread");
	Ok(())
}
