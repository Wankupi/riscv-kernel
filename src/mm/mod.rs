pub mod vm;

pub mod allocator;

use core::alloc::{GlobalAlloc, Layout};

use allocator::simple::SimpleAllocator;
pub static simple_allocator: SimpleAllocator = SimpleAllocator::new();

use allocator::buddy::BuddyAllocator;
#[global_allocator]
pub static mut buddy_allocator: BuddyAllocator = BuddyAllocator::new();

static mut use_buddy: bool = false;

pub fn alloc(layout: Layout) -> *mut u8 {
	unsafe {
		if use_buddy {
			buddy_allocator.alloc(layout)
		} else {
			simple_allocator.alloc(layout)
		}
	}
}

pub fn dealloc(ptr: *mut u8, layout: Layout) {
	unsafe {
		if use_buddy {
			buddy_allocator.dealloc(ptr, layout)
		} else {
			simple_allocator.dealloc(ptr, layout)
		}
	}
}

pub fn change_to_buddy() {
	unsafe {
		use_buddy = true;
	}
}