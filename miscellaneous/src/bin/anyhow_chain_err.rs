use anyhow::{Context, Result};

fn process_data(data: &str) -> Result<()> {
	let parsed = data.parse::<i32>().with_context(|| format!("Failed to parse data: {}", data))?;
	println!("Parse data: {}", parsed);
	Ok(())
}

fn main() -> Result<()>{
  process_data("123")?;
  process_data("ad")?;
  Ok(())
}
