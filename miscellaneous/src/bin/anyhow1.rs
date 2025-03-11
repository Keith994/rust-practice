
use anyhow::{anyhow, Context, Result};

fn step1() -> Result<()> {
  Err(anyhow!("step1 failed"))
}

fn  step2() -> Result<()> {
  Err(anyhow!("step2 failed"))
}

fn main() -> Result<()> {
  let result = std::fs::read_to_string("noexist.txt")
    .with_context(|| "Failed to read file")?;
  println!("{}", result);
  step1().context("Failed at step 1")?;
  step2().context("Failed at step 2")?;
  Ok(())
}
