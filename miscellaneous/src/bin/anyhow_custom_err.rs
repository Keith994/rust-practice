use anyhow::{Result, anyhow};

fn divide(a: i32, b: i32) -> Result<i32> {
	if b == 0 {
		return Err(anyhow!("Dividing by zero"));
	}
	Ok(a / b)
}

fn main() -> Result<()> {
	let result = divide(2, 0)?;
	println!("Result: {}", result);
	Ok(())
}
