// 基本用法

use anyhow::{Context, Result};

fn read_file(path: &str) -> Result<String> {
	let content =
		std::fs::read_to_string(path).with_context(|| format!("Failed to read file: '{}'", path))?;
	Ok(content)
}

fn main() -> Result<()> {
	let content = read_file("example.txt")?;
	println!("File content: {}", content);
	Ok(())
}
