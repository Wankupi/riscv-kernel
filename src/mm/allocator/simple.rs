use crate::{
	config::PAGE_SIZE,
	mm::allocator::use_buddy,
	sync::{LockGuard, SpinLock},
};
use core::{
	alloc::{GlobalAlloc, Layout},
	cell::UnsafeCell,
	cmp::max,
};

pub struct SimpleAllocator {
	next: UnsafeCell<usize>,
	start: UnsafeCell<usize>,
	end: UnsafeCell<usize>,
	mutex: SpinLock,
}

unsafe impl Sync for SimpleAllocator {}

unsafe impl GlobalAlloc for SimpleAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		self.dbg_report();
		let align = layout.align();
		let _lock = LockGuard::new(&self.mutex);
		let ret = (*self.next.get() + align - 1) & !(align - 1);
		self.next.get().write(ret + layout.size());
		if self.next.get().read() > self.end.get().read() {
			if use_buddy {
				self.dbg_report();
				panic!("SimpleAllocator OOM {:x}, align {}", layout.size(), align);
			}
			self.enlarge(0);
		}
		return ret as *mut u8;
	}
	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

impl SimpleAllocator {
	pub const fn new() -> SimpleAllocator {
		return SimpleAllocator {
			next: UnsafeCell::new(0),
			start: UnsafeCell::new(0),
			end: UnsafeCell::new(0),
			mutex: SpinLock::new(),
		};
	}
	pub fn init(&self, start: usize, size: usize) {
		unsafe {
			use crate::PAGE_SIZE;
			let check_aligned = |x: usize| x & (PAGE_SIZE - 1) == 0;
			let end: usize = start + size;
			assert!(check_aligned(start) && check_aligned(end));
			self.next.get().write(start);
			self.start.get().write(start);
			self.end.get().write(end);
			log!(
				"SimpleAllocator: init: start: {:#x}, end: {:#x} ({} blocks)",
				start,
				end,
				size / PAGE_SIZE
			);
		}
	}
	pub fn get_control_range(&self) -> (usize, usize) {
		unsafe { (self.start.get().read(), self.end.get().read()) }
	}
	pub fn dbg_report(&self) {
		let have = unsafe { self.end.get().read() - self.start.get().read() };
		let used = unsafe { self.next.get().read() - self.start.get().read() };
		log!(
			"SimpleAllocator: have: {:x}, used: {:x}, remain: {:x}",
			have,
			used,
			have - used
		);
	}
	pub fn can_alloc(&self, layout: Layout) -> bool {
		let align = layout.align();
		let _lock = LockGuard::new(&self.mutex);
		unsafe {
			let ret = (*self.next.get() + align - 1) & !(align - 1);
			ret + layout.size() <= self.end.get().read()
		}
	}
	pub unsafe fn enlarge(&self, size: usize) {
		assert!(!use_buddy);
		let new_end = max(self.next.get().read() + size, self.end.get().read());
		let new_end_aligned = (new_end + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
		self.end.get().write(new_end_aligned);
	}
}
