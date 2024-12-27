use std::collections::HashMap;
use std::hash::Hash;

#[allow(unused)]
fn main() {
	println!("err_lifetime2.rs");

  // 生命周期'm活得至少跟get_default一样长
  // 
	fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
	where
		K: Clone + Eq + Hash,
		V: Default,
	{
	  // 编译器未能精确地判断出某个可变借用不再需要，反而谨慎的给该借用安排了一个很大的作用域
	  //
	  // map是第一次可变借用 用完应该是不会用了的，但是上述原因已经指出
	  // 所以第一次的可变借用会持续到match快结束
		// match map.get_mut(&key) { // 这里要进行更改
		// 	Some(value) => value,
		// 	None => {
		// 	  // map的第二次可变借用
		// 	  // **可变借用的生命周期里只能有一个可变借用**
		// 		map.insert(key.clone(), V::default());
		// 		map.get_mut(&key).unwrap()
		// 	}
		// }

    
    let mut_v = map.get(&key);
		match mut_v { 
			Some(value) => value,
			None => {
				map.insert(key.clone(), V::default());
				map.get_mut(&key).unwrap()
			}
		}
	}
}
