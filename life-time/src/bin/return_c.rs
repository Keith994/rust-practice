fn make_tester(answer: String) -> impl Fn(&str) -> bool {
	// add code here
	move |challenge| answer == challenge
}

fn main() {
	let t = make_tester(String::from("hello"));
	let b = t("hello");
	println!("{b}");
	let b = t("hello");
	println!("{b}");
}
