use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
	let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

	{
		let mut map: RefMut<'_, _> = shared_map.borrow_mut();
		map.insert("africa", 92388);
		map.insert("kyoto", 11837);
		map.insert("piccadilly", 11826);
		map.insert("marbles", 38);
	}

	let total: i32 = shared_map.borrow().values().sum();
	println!("{total}");

	let rci: Rc<RefCell<_>> = Rc::new(RefCell::new(123));
	let mut mutr: RefMut<'_, i32> = rci.borrow_mut();
	*mutr = 10;
	println!("{mutr}");
}
