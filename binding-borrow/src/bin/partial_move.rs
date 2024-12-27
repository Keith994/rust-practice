fn main() {
	#[derive(Debug)]
	struct Person {
		name: String,
		age: u8,
	}
	#[allow(dead_code)]
	fn drop_person(_: Person) {}

	let mut person = Person { name: String::from("Alice"), age: 20 };

	let Person { name, ref age } = person;

	println!("The person's age is {}", age);
	println!("The person's name is {}", name);

	// 重新绑定
	person.name = String::from("Keith");

	println!("The person's struct is {:?}", person);

	// 不能drop，age还在进行借用
	// drop_person(person);

	println!("Ther person's age from person struct is {}", age);

  #[derive(Debug)]
  #[allow(dead_code)]
	struct PersonStack {
		code: i32,
		age: u8,
	}

	//  // 结构的一定是在堆上的
	// let ps = PersonStack { code: 1, age: 30 };
	// let ps1 = ps;
	//
	// println!("ps struct {:?}", ps);
	

}
