const MAX_ORDER: usize = 11;

use core::{
	alloc::{GlobalAlloc, Layout},
	mem::size_of,
};

use crate::{
	arch::mm::{PAGE_SIZE, PAGE_SIZE_BITS},
	lang::{Bitmap, UnsafeArray},
	mm::vm,
	sync::{self, SpinLock},
};

// last == next == nullptr mean this area is being used
#[derive(Copy, Clone)]
struct MemAreaStatus {
	last: *mut MemAreaStatus,
	next: *mut MemAreaStatus,
	block_id: usize,
}
impl MemAreaStatus {
	pub const fn new() -> Self {
		Self {
			last: 0 as *mut MemAreaStatus,
			next: 0 as *mut MemAreaStatus,
			block_id: 0,
		}
	}
}

pub struct BuddyAllocator {
	phys_offset: usize,
	// lists: [MemAreaStatus; MAX_ORDER],
	lists: [MemAreaStatus; MAX_ORDER + 1],
	bitmaps: [Bitmap; MAX_ORDER + 1],
	meta: UnsafeArray<MemAreaStatus>, // point to a dynamic place, but would not be changed
	mutex: SpinLock,
}

impl BuddyAllocator {
	pub const fn new() -> Self {
		Self {
			phys_offset: 0,
			lists: [MemAreaStatus::new(); MAX_ORDER + 1],
			bitmaps: [Bitmap::new(); MAX_ORDER + 1],
			meta: UnsafeArray::new(),
			mutex: SpinLock::new(),
		}
	}
}

impl BuddyAllocator {
	fn estimate_meta_size(size: usize) -> usize {
		let blocks = size >> PAGE_SIZE_BITS;
		// bitmap
		// blocks * (1 + 1/2 + 1/4 + ...)
		let bitmap_size = (blocks + size_of::<usize>() - 1) / size_of::<usize>() * 2;
		// meta
		let meta_size = blocks * size_of::<MemAreaStatus>();
		return bitmap_size + meta_size;
	}
	// // test: MaxOrder = 2, order = 1, then 0xx 10x 110, base = 100
	// fn get_meta_index_from_phys_offset(order: usize, phys_offset: usize) -> usize {
	// 	let base: usize = ((1 << order) - 1) << (MAX_ORDER - order + 1);
	// 	let offset = phys_offset >> (order + PAGE_SIZE_BITS);
	// 	return base + offset;
	// }
	fn get_meta_index_from_block_id(order: usize, block_id: usize) -> usize {
		// let base: usize = ((1 << order) - 1) << (MAX_ORDER - order + 1);
		// let offset = block_id >> order;
		// return base + offset;
		return block_id;
	}
	fn get_meta_ref_from_block_id(&self, order: usize, block_id: usize) -> &mut MemAreaStatus {
		let index = Self::get_meta_index_from_block_id(order, block_id);
		self.meta.get_mut(index)
	}
	fn get_list_head(&self, order: usize) -> &mut MemAreaStatus {
		let list = &self.lists[order];
		let ptr = list as *const MemAreaStatus as *mut MemAreaStatus;
		unsafe { ptr.as_mut().unwrap() }
	}
	fn add_list_tail(&mut self, order: usize, block_id: usize) {
		let meta = self.get_meta_ref_from_block_id(order, block_id);
		let list = self.get_list_head(order);
		meta.block_id = block_id;
		meta.last = list.last;
		meta.next = 0 as *mut MemAreaStatus;
		if meta.last != 0 as *mut MemAreaStatus {
			unsafe {
				(*meta.last).next = meta;
			}
		}
		list.last = meta;
	}
	fn add_list_head(&self, order: usize, block_id: usize) {
		let meta = self.get_meta_ref_from_block_id(order, block_id);
		let list = self.get_list_head(order);
		meta.block_id = block_id;
		meta.last = 0 as *mut MemAreaStatus;
		meta.next = list.next;
		if meta.next != 0 as *mut MemAreaStatus {
			unsafe {
				(*meta.next).last = meta;
			}
		}
		list.next = meta;
	}
	pub fn init_lists(&mut self) {
		for i in 0..MAX_ORDER {
			self.lists[i].last = 0 as *mut MemAreaStatus;
			self.lists[i].next = 0 as *mut MemAreaStatus;
		}
	}
	pub fn init_bitmap(&mut self, blocks: usize, current_start: &mut usize) {
		for i in 0..=MAX_ORDER {
			let block_cnt = blocks >> i;
			let use_mem = (block_cnt + size_of::<usize>() - 1) & !(size_of::<usize>() - 1);
			let addr = *current_start;
			self.bitmaps[i].init(addr as *mut usize, use_mem);
			*current_start += use_mem;
		}
	}
	pub fn init_nodes_mem(&mut self, blocks: usize, current_start: &mut usize) {
		*current_start = (*current_start + size_of::<usize>() - 1) & !(size_of::<usize>() - 1);
		self.meta.init(*current_start as *mut MemAreaStatus);
		*current_start += blocks * size_of::<MemAreaStatus>();
	}
	pub fn init_add_nodes(&mut self, blocks: usize) {
		let mut current_start: usize = 0;
		for i in (0..=MAX_ORDER).rev() {
			let block_cnt = 1 << i;
			while current_start + block_cnt <= blocks {
				self.add_list_tail(i, current_start);
				current_start += block_cnt;
			}
		}
	}
	pub fn init(&mut self, phys_begin: usize, size: usize) {
		let meta_size = Self::estimate_meta_size(size);
		let reserved_size = (meta_size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
		// let meta_begin = phys_begin + meta_size;
		let available_size = size - reserved_size;
		let available_blocks = available_size >> PAGE_SIZE_BITS;
		let mut reserve_alloc = phys_begin + vm::get_kernel_v2p_offset();
		self.phys_offset = phys_begin + reserved_size;
		self.mutex.init();
		self.init_lists();
		self.init_bitmap(available_blocks, &mut reserve_alloc);
		self.init_nodes_mem(available_blocks, &mut reserve_alloc);
		self.init_add_nodes(available_blocks);
	}
	fn get_alloc_order(mut size: usize) -> usize {
		fn lowbit(x: usize) -> usize {
			x & (-(x as isize) as usize)
		}
		size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
		while size != lowbit(size) {
			size += lowbit(size);
		}
		size >>= PAGE_SIZE_BITS;
		let mut res = 0;
		while size != 1 {
			size >>= 1;
			res += 1;
		}
		res
	}
	fn toggle_bit(&self, order: usize, block_id: usize) {
		self.bitmaps[order].toggle(block_id >> order);
	}
	fn _down_block(&self, order: usize) {
		if order > MAX_ORDER {
			panic!("order too large");
		}
		let head = self.get_list_head(order);
		if head.next == 0 as *mut MemAreaStatus {
			self._down_block(order + 1);
		}
		let meta = unsafe { &mut *head.next };
		head.next = meta.next;
		meta.next = 0 as *mut MemAreaStatus;
		meta.last = 0 as *mut MemAreaStatus;
		let block_id = meta.block_id;
		self.toggle_bit(order, block_id);
		let split_A = block_id;
		let split_B = block_id + (1 << (order - 1));
		self.add_list_head(order - 1, split_A);
		self.add_list_head(order - 1, split_B);
	}
	fn _alloc_block(&self, order: usize) -> *mut u8 {
		let head = self.get_list_head(order);
		if head.next == 0 as *mut MemAreaStatus {
			self._down_block(order + 1);
			// if could not down, return fail
			if head.next == 0 as *mut MemAreaStatus {
				return 0 as *mut u8;
			}
		}
		let meta = unsafe { &mut *head.next };
		head.next = meta.next;
		meta.next = 0 as *mut MemAreaStatus;
		meta.last = 0 as *mut MemAreaStatus;
		let block_id = meta.block_id;
		self.bitmaps[order].set(block_id);
		let ret = (block_id << PAGE_SIZE_BITS) + self.phys_offset;
		ret as *mut u8
	}
	fn _dealloc_block(&self, ptr: *mut u8, order: usize) {
		let block_id = (ptr as usize - self.phys_offset) >> PAGE_SIZE_BITS;
		self.toggle_bit(order, block_id);
		self.add_list_head(order, block_id);
		// TODO merge
	}
}

unsafe impl GlobalAlloc for BuddyAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let order = Self::get_alloc_order(layout.size());
		let _guard = sync::LockGuard::new(&self.mutex);
		self._alloc_block(order)
	}
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let order = Self::get_alloc_order(layout.size());
		let _guard = sync::LockGuard::new(&self.mutex);
		self._dealloc_block(ptr, order);
	}
}
