#![allow(unused)]
fn main() {
	use std::borrow::Cow;

	fn abs_all(input: &mut Cow<'_, [i32]>) {
		for i in 0..input.len() {
			let v = input[i]; // i32会发生Copy行为，但不影响
			if v < 0 {
				// Clones into a vector if not already owned.
				input.to_mut()[i] = -v;
			}
		}
	}

	// No clone occurs because `input` doesn't need to be mutated.
	let slice = [0, 1, 2]; // 来源是一个切片所有者，且是不可变的，即不能直接修改
	let mut input = Cow::from(&slice[..]); // 通过Cow包裹后， input 是一个新的所有者，并且是可变的
	abs_all(&mut input); // 传递Cow的可变引用到函数里面，表示Cow可变更数据
	//
	println!("no clone occurs "); // 虽然Cow是可变的，但没有真正变更内部封装的数据，所以没有进行复制，使用的还是原来的数据
	println!("slice {:?}", slice);
	println!("input {:?}", input);

	// Clone occurs because `input` needs to be mutated.
	let slice = [-1, 0, 1];
	let mut input = Cow::from(&slice);
	abs_all(&mut input); // 在这个情况下，有-1，需要变更 来源数据
	// ，所以Cow进行了一次clone行为，复制了全新的[-1,0,1]，在此基础上修改不会影响原来的数据
	println!("clone occurs");
	println!("slice {:?}", slice);
	println!("input {:?}", input);

	// No clone occurs because `input` is already owned.
	let mut input = Cow::from(vec![-1, 0, 1]); // 对封装数据有直接的所有权，所以会在封装数据上进行修改，不会进行clone行为
	abs_all(&mut input);
	println!("no clone occurs");
	println!("slice {:?}", slice);
	println!("input {:?}", input);
}
