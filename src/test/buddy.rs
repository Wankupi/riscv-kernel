use core::alloc::Layout;
use crate::{alloc, dealloc};

pub fn test_buddy() {
	let layout = Layout::from_size_align(4096, 4096).unwrap();
	let mut a = [0 as * mut u8; 10];
	let n = 3;
	for i in 0..n {
		a[i] = alloc(layout);
		info!("alloc  : {:16x}", a[i] as usize);
	}
	for i in 0..n {
		dealloc(a[i], layout);
		info!("dealloc: {:16x}", a[i] as usize);
	}

	for i in 0..n {
		a[i] = alloc(layout);
		info!("alloc  : {:16x}", a[i] as usize);
	}
	for i in 0..n {
		dealloc(a[i], layout);
		info!("dealloc: {:16x}", a[i] as usize);
	}
}
