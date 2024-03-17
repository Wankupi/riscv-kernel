pub const PAGE_SIZE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;
pub const PA_WIDTH: usize = 56;
pub const PA_SIZE: usize = 1 << PA_WIDTH;
pub const PPN_WIDTH: usize = PA_WIDTH - PAGE_SIZE_BITS;
pub const PPN_SIZE: usize = 1 << PPN_WIDTH;
pub const VT_MAP_SIZE: usize = 4096 / 8;

use bitflags::bitflags;
use crate::mm::declare::PhysPageNum;

bitflags! {
	pub struct PTE: usize {
		const V = 1 << 0;
		const R = 1 << 1;
		const W = 1 << 2;
		const X = 1 << 3;
		const U = 1 << 4;
		const G = 1 << 5;
		const A = 1 << 6;
		const D = 1 << 7;
	}
}

pub struct PageTableEntry {
	pub bits: usize,
}

impl PageTableEntry {
	pub fn get_valid(&self) -> bool {
		self.bits & 1 != 0
	}
	pub fn set_flags(&mut self, flags: PTE) {
		self.bits = (self.bits & !0xff) | flags.bits();
	}
	pub fn get_ppn(&self) -> PhysPageNum {
		PhysPageNum(self.bits >> 10 & ((1 << 44) - 1))
	}
	pub fn set_ppn(&mut self, ppn: PhysPageNum) {
		self.bits = (self.bits & !((1 << 44) - 1) << 10) | (ppn.0 << 10);
	}
	pub fn enable(&mut self) {
		self.bits |= 1;
	}
	pub fn disable(&mut self) {
		self.bits &= !1;
	}
	pub fn clear(&mut self) {
		self.bits = 0;
	}
}
