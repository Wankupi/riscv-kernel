pub mod buddy;
pub mod simple;

use core::alloc::{GlobalAlloc, Layout};

use simple::SimpleAllocator;
pub static simple_allocator: SimpleAllocator = SimpleAllocator::new();

use buddy::BuddyAllocator;

use crate::{config::PAGE_SIZE, driver::fdt::get_fdt};
#[global_allocator]
pub static mut buddy_allocator: BuddyAllocator = BuddyAllocator::new();

static mut use_buddy: bool = false;

const IS_DEBUG: bool = false;

/// @return physical address
pub fn alloc(layout: Layout) -> *mut u8 {
	let r = unsafe {
		if use_buddy {
			buddy_allocator.alloc(layout)
		} else {
			simple_allocator.alloc(layout)
		}
	};
	if IS_DEBUG {
		println!("alloc: [{:p}, {:p})", r, r.wrapping_add(layout.size()));
	}
	r
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
	if IS_DEBUG {
		println!("alloc: [{:p}, {:p})", ptr, ptr.wrapping_add(layout.size()));
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
	let region = get_fdt().memory().regions().next().unwrap();
	unsafe {
		buddy_allocator.init(
			simple_allocator.get_control_range().1,
			region.starting_address.wrapping_add(region.size.unwrap()) as usize,
		);
		use_buddy = true;
	}
}

pub fn pre_alloc_buddy() {
	let region = get_fdt().memory().regions().next().unwrap();
	let mem_end = region.starting_address.wrapping_add(region.size.unwrap()) as usize;
	let buddy_start = (simple_allocator.get_control_range().1 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
	let size = mem_end - buddy_start;
	let meta_size = BuddyAllocator::estimate_meta_size(size);
	unsafe { simple_allocator.enlarge(meta_size) };
}
