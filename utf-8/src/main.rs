use std::{fs::File, io::ErrorKind};

fn main() {
	let s = "न你好这是什么";
	println!("Hello, world!, {}", s.len());
	let b = s.as_bytes();
	println!("Hello, world!, {:0x?}", b); // 16进制表示
	//
	//
	let c = '好';
	let c1 = 'A';
	println!("{c}{}{}", c.len_utf8(), c1.len_utf8());

	println!("{}", &s[3..6]); // 你

	let mut b = s.chars();
	for b1 in &mut b {
		// 相当于b.into_iter()
		println!("{:?}", b1);
	}
	println!("{:?}", b); // 已经被消耗
	//
	let greeting_file = File::open("hello.txt").unwrap_or_else(|e| match e.kind() {
		ErrorKind::NotFound => File::create("hello.txt").unwrap_or_else(|e1| {
			panic!("Problem creating the file: {e1:?}");
		}),
		_ => panic!("Problem happen {e:?}"),
	});
}
