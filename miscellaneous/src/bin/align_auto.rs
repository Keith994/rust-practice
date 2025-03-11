#[repr(C)]
struct Data {
	a: u8,  // 1个字节
	b: u32, // 4个字节
	c: u16, // 2个字节
	// 填充一个字节
	d: u64, // 8个字节
}

// 内存对齐会自动优化字段顺序以对齐
#[repr(C)] // 这个会组织rust自动对齐
struct Data1 {
	a: u8,  // 1个字节
	b: u32, // 4个字节
	c: u16, // 3个字节
}

fn main() {
	println!("Data大小: {}字节", std::mem::size_of::<Data>()); // 16个字节
	println!("Data大小: {}字节", std::mem::size_of::<Data1>()); // 8个字节
}
