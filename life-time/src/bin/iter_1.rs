fn main() {
	for c in "SuRPRISE INbOUND".chars().filter(|x| x.is_lowercase()).flat_map(|x| x.to_uppercase()) {
		print!("{}", c);
	}
	println!();
	let cc = "SuRPRISE INbOUND"
		.chars()
		.filter(|x| x.is_uppercase())
		.flat_map(|x| x.to_lowercase())
		.collect::<Vec<char>>();
	println!("{cc:?}");
}
