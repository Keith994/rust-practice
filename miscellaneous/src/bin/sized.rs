#[allow(dead_code)]
// 每个T类型参数都会有一个Sized(这个Sized特征是大小已经知道的)特征绑定，就是需要知道T的确切大小
struct Foo<T>(T); 

#[allow(dead_code)]
// ?Sized可以移除T的Sized特征绑定，编译器就不会检查T的大小了
struct Bar<T: ?Sized>(T);

// [i32]是一个切片，不知道确切大小，切片并没有实现Sized特征
// struct FooUse(Foo<[i32]>);

#[allow(dead_code)]
// i32是已知大小的，即实现了Sized特征
struct FooUse1(Foo<i32>);

#[allow(dead_code)]
struct BarUse(Bar<[i32]>);


// trait的隐式Self类型没有Sized界限，即它是默认移除Sized特征的

fn main() {
  println!("hello");
	todo!();
}
