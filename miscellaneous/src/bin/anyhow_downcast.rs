use anyhow::{Error, Result, anyhow};

fn main() -> Result<()> {
	let err: Error = anyhow!("Custom Err");

	if let Some(err) = err.downcast_ref::<String>() {
		println!("Caught a String error: {}", err);
	} else {
		println!("Error is not a String");
	}
	Ok(())
}
