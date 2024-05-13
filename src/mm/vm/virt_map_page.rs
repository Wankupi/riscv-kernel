use core::alloc::Layout;

use alloc::boxed::Box;

use super::page_table_entry::PageTableEntry;
use crate::alloc;
use crate::PAGE_SIZE_BITS;
use crate::VT_MAP_SIZE;

#[repr(align(4096))]
pub struct VirtMapPage {
	pub entries: [PageTableEntry; VT_MAP_SIZE],
}

impl VirtMapPage {
	fn clear(&mut self) {
		for e in self.entries.iter_mut() {
			e.clear();
		}
	}
	pub fn create() -> *mut VirtMapPage {
		let layout = Layout::new::<VirtMapPage>();
		let k_vt = unsafe { &mut *(alloc(layout) as *mut VirtMapPage) };
		k_vt.clear();
		k_vt
	}
	pub fn create_ref() -> &'static mut VirtMapPage {
		unsafe { &mut *Self::create() }
	}
	pub fn create_box() -> Box<VirtMapPage> {
		unsafe { Box::from_raw(Self::create()) }
	}
	pub fn to_satp(&self) -> usize {
		(self as *const Self as usize >> PAGE_SIZE_BITS) | (8 << 60)
	}
}
impl Default for VirtMapPage {
	fn default() -> Self {
		Self {
			entries: [PageTableEntry::default(); VT_MAP_SIZE],
		}
	}
}
