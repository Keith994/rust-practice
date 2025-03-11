fn for_each_planet<F>(f: F)
where
	F: Fn(&'static str) + 'static,
{
  f("Earch");
  f("Mars");
  f("Jupiter");
}

fn main() {
  let greeting = String::from("Good to see you!");
  for_each_planet(|planet| println!("Hello {planet},{greeting}")); // 借用了greeting，但是闭包是静态绑定的，则闭包可能活得比main函数久，闭包借用的greeting可能会失效
}
