fn main() {
	squares();

	to_upper();

	collect_stru();

	chain_use();

	map_option();

	map_external_var();

	map_move_owned();
}

fn squares() {
	let mut nums = vec![1, 2, 3];
	let squares: Vec<i32> = nums.iter().map(|x| x * 2).collect();
	println!("{:?}", nums);

	nums
		.iter_mut()
		.map(|x| {
			*x += 2;
		}) // iter_mut的map返回()值
		.for_each(|_| {});

	println!("{:?}", nums);
	println!("{:?}", squares);
}

fn to_upper() {
	let words = vec!["hello", "world"];
	let uppercase: Vec<String> = words.iter().map(|s| s.to_uppercase()).collect();
	assert_eq!(uppercase, vec!["HELLO", "WORLD"]);
	assert_ne!(uppercase, words);
}

fn collect_stru() {
	#[derive(Debug)]
	struct User {
		name: String,
		age: u8,
	}

	let users = [User { name: "Alice".into(), age: 30 }, User { name: "Bob".into(), age: 25 }];

	let name: Vec<String> = users.iter().map(|u| u.name.clone()).collect();

	println!("{:?}", name);
}

fn chain_use() {
	let nums = [1, 2, 3, 4, 5];

	let even_sqaures: Vec<i32> = nums.iter().filter(|x| *x % 2 == 0).map(|x| x * x).collect();

	println!("{:?}", even_sqaures)
}

fn map_option() {
	let maybe_numbers = [Some(1), None, Some(3)];
	let incremented: Vec<Option<i32>> = maybe_numbers.iter().map(|opt| opt.map(|x| x + 1)).collect();
	println!("{:?}", incremented);
}

fn map_external_var() {
	let factor = 2;
	let nums = [1, 2, 3];
	let scaled: Vec<i32> = nums.iter().map(|x| x * factor).collect();

	println!("{:?}", scaled);
}

fn map_move_owned() {
	let strings = ["a".to_string(), "b".into()];
	let lengths: Vec<usize> = strings.into_iter().map(|s| s.len()).collect();
	println!("{:?}", lengths);
}
