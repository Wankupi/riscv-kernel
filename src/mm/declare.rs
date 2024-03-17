use core::arch::asm;

use crate::arch::mm::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

impl PhysAddr {
	pub fn floor(&self) -> PhysPageNum {
		PhysPageNum(self.0 / PAGE_SIZE)
	}
	pub fn ceil(&self) -> PhysPageNum {
		PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
	}
}

impl From<usize> for PhysAddr {
	fn from(v: usize) -> Self {
		Self(v & (PA_SIZE - 1))
	}
}

impl From<usize> for PhysPageNum {
	fn from(v: usize) -> Self {
		Self(v & (PPN_SIZE - 1))
	}
}

impl From<PhysAddr> for usize {
	fn from(v: PhysAddr) -> Self {
		v.0
	}
}

impl From<PhysPageNum> for usize {
	fn from(v: PhysPageNum) -> Self {
		v.0
	}
}

impl PhysAddr {
	pub fn page_offset(&self) -> usize {
		self.0 & (PAGE_SIZE - 1)
	}
}

impl From<PhysAddr> for PhysPageNum {
	fn from(v: PhysAddr) -> Self {
		// assert_eq!(v.page_offset(), 0);
		v.floor()
	}
}

impl From<PhysPageNum> for PhysAddr {
	fn from(v: PhysPageNum) -> Self {
		Self(v.0 << PAGE_SIZE_BITS)
	}
}

#[repr(align(4096))]
struct VirtMapPage {
	entries: [PageTableEntry; VT_MAP_SIZE],
}

impl VirtMapPage {
	fn clear(&mut self) {
		for e in self.entries.iter_mut() {
			e.clear();
		}
	}
	unsafe fn create() -> *mut VirtMapPage {
		let layout = Layout::new::<VirtMapPage>();
		let k_vt = &mut *(alloc(layout) as *mut VirtMapPage);
		k_vt.clear();
		k_vt
	}
	fn register(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTE) {
		let idx = [
			(vpn.0 >> (9 + 9 + 12)) & 0x1ff,
			(vpn.0 >> (9 + 12)) & 0x1ff,
			(vpn.0 >> (12)) & 0x1ff,
		];
		// print!("idx = {:x} {:x} {:x}\n", idx[0], idx[1], idx[2]);
		let mut ptr = self;
		for i in 0..2 {
			let k = idx[i];
			let entry = &mut ptr.entries[k];
			if !entry.get_valid() {
				let p = unsafe { Self::create() };
				entry.set_ppn(PhysPageNum::from(PhysAddr::from(p as usize)));
				entry.enable();
				print!("create new page table for vpn = {:x} at level {}\n", k, i);
			}
			let p = entry.get_ppn();
			ptr = unsafe { &mut *(PhysAddr::from(p).0 as *mut VirtMapPage) };
		}
		let entry = &mut ptr.entries[idx[2]];
		entry.set_ppn(ppn);
		entry.set_flags(flags);
		entry.enable();
	}
}

use alloc::alloc::{alloc, Layout};
static mut k_vt_ptr: *mut VirtMapPage = 0 as *mut VirtMapPage;

use crate::print;
pub fn init_vm(start: PhysPageNum, end: PhysPageNum) {
	print!("start create k_vt_ptr\n");
	unsafe {
		k_vt_ptr = VirtMapPage::create();
	}
	print!("finish create k_vt_ptr\n");
	print!("start ppn = {:x}, end ppn = {:x}\n", start.0, end.0);
	for i in start.0..=(end.0 + 10) {
		// print!("i = {:x}\n", i);
		unsafe {
			(*k_vt_ptr).register(VirtPageNum(i), PhysPageNum(i), PTE::V | PTE::R | PTE::W | PTE::X);
		}
	}
	print!("finish set vtable\n");
	let satp = unsafe { (8 << 60) | PhysPageNum::from(PhysAddr(k_vt_ptr as usize)).0 };
	unsafe {
		asm!("sfence.vma");
		asm!("csrw satp, {}", in(reg) satp);
	}
}
