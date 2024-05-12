use core::sync::atomic::AtomicI32;

use alloc::boxed::Box;

struct ArcBox<T> {
	ref_count: * const AtomicI32,
	data: *mut T,
}

impl<T> Default for ArcBox<T> {
	fn default() -> Self {
		Self {
			ref_count: core::ptr::null(),
			data: core::ptr::null_mut(),
		}
	}
}

impl<T> ArcBox<T> {
	// fn new() -> Self {
	// 	let r = Box::new(AtomicI32::new(1));
	// 	Self {
	// 		ref_count: ,
	// 	}
	// }
	fn borrow(&self) -> &T {
		unsafe { &*self.data }
	}
	fn borrow_mut(&self) -> &mut T {
		unsafe { &mut *self.data }
	}
}

// impl<T> Clone for ArcBox<T> {
// 	fn clone(&self) -> Self {
// 		self.ref_count.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
// 		Self {
// 			ref_count: self.ref_count.clone(),
// 			data: self.data,
// 		}
// 	}
// }
