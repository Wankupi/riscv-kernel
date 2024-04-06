use core::{
	alloc::{GlobalAlloc, Layout},
	cell::UnsafeCell,
	sync::atomic::AtomicUsize,
};

pub struct SimpleAllocator {
	next: UnsafeCell<usize>,
	mutex: AtomicUsize,
}

unsafe impl Sync for SimpleAllocator {}
use core::sync::atomic::Ordering::SeqCst;

use crate::success;
unsafe impl GlobalAlloc for SimpleAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let align = layout.align();
		loop {
			let res = self.mutex.compare_exchange(0, 1, SeqCst, SeqCst);
			if res.is_ok() {
				break;
			}
		}
		let ret = (*self.next.get() + align - 1) & !(align - 1);
		self.next.get().write(ret + layout.size());
		self.mutex.store(0, SeqCst);
		
		// print!("alloc: {:x}, size={}\n", ret, layout.size());
		return ret as *mut u8;
	}
	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

impl SimpleAllocator {
	pub const fn new() -> SimpleAllocator {
		return SimpleAllocator {
			next: UnsafeCell::new(0),
			mutex: AtomicUsize::new(0),
		};
	}
	pub fn init(&self, start: usize) {
		unsafe {
			use crate::arch::mm::PAGE_SIZE;
			self.next.get().write((start + (PAGE_SIZE - 1)) & !(PAGE_SIZE - 1));
		}
		success!("initialize allocator");
	}
}
