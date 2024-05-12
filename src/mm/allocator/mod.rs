pub mod buddy;
pub mod simple;

use core::alloc::{GlobalAlloc, Layout};

use simple::SimpleAllocator;
pub static simple_allocator: SimpleAllocator = SimpleAllocator::new();

use buddy::BuddyAllocator;
#[global_allocator]
pub static mut buddy_allocator: BuddyAllocator = BuddyAllocator::new();

static mut use_buddy: bool = false;

/// @return physical address
pub fn alloc(layout: Layout) -> *mut u8 {
	unsafe {
		if use_buddy {
			buddy_allocator.alloc(layout)
		} else {
			simple_allocator.alloc(layout)
		}
	}
}
/// @params ptr: physical address
pub fn dealloc(ptr: *mut u8, layout: Layout) {
	unsafe {
		if use_buddy {
			buddy_allocator.dealloc(ptr, layout)
		} else {
			simple_allocator.dealloc(ptr, layout)
		}
	}
}

pub fn alloc_size_align(size: usize, align: usize) -> *mut u8 {
	let layout = Layout::from_size_align(size, align).unwrap();
	alloc(layout)
}
pub fn dealloc_size_align(ptr: *mut u8, size: usize, align: usize) {
	let layout = Layout::from_size_align(size, align).unwrap();
	dealloc(ptr, layout)
}
pub fn alloc_size(size: usize) -> *mut u8 {
	alloc_size_align(size, 4)
}
pub fn dealloc_size(ptr: *mut u8, size: usize) {
	dealloc_size_align(ptr, size, 4)
}

pub fn change_to_buddy() {
	unsafe {
		use_buddy = true;
	}
}
