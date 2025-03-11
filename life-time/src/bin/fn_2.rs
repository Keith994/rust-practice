fn foobar<F>(mut f: F)
where
	F: FnMut(i32) -> i32,
{
	println!("{}", f(2));
}

fn foobar1<F>(f: F)
where
	F: Fn(i32) -> i32,
{
	f(2);
}

fn foobar2<F>(f: F)
where
	F: FnOnce(i32) -> String,
{
	f(10);
	f(20);
}

fn main() {
	foobar(|x| x * 2);
	let mut acc = 2;
	foobar1(|x| {
		acc += 1;
		x * acc
	});
	let s = String::from("alright");
	foobar2(|_| s);
	println!("{s}");
}
