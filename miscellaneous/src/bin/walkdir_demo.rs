use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::WalkDir;

fn main() {
	WalkDir::new("/home/keith/").max_depth(2).into_iter().par_bridge().for_each(|entry| {
		if let Ok(e) = entry {
			println!("File: {:?}", e.file_name())
		}
	});
}
