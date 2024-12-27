#[derive(Debug)]
struct Foo;

impl Foo {
	fn mutate_and_share(&mut self) -> &Self {
		&*self
	}

  #[allow(dead_code)]
	fn share(&self) {}
}

fn main() {
	let mut foo = Foo;
	let loan = foo.mutate_and_share();
	// foo.share();

	// loan是不可变引用，一直引用到了这里
	// mutate_and_share的foo调用是可变引用
	// 他们都引用了相同的生命周期'c
	//
	// share的foo调用是不可变引用，且他的生命周期'd比'c小
	//
	// rust中，在可变引用的生命周期范围内，只能有且只有一个可变引用，且不能有不可变引用
	println!("{:?}", loan);

	// 'b: {
	//   'c: {
	//     let loan: &'c Foo = Foo::mutate_and_share::<'c>(&'c mut foo);
	//     'd: {
	//       Foo::share::<'d>(&'d foo);
	//     }
	//      println!("{:?}", loan);
	//   }
	// }
}
