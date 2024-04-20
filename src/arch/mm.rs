pub const PAGE_SIZE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;
pub const PA_WIDTH: usize = 56;
pub const PA_SIZE: usize = 1 << PA_WIDTH;
pub const PPN_WIDTH: usize = PA_WIDTH - PAGE_SIZE_BITS;
pub const PPN_SIZE: usize = 1 << PPN_WIDTH;
pub const VT_MAP_SIZE: usize = 4096 / 8;

use bitflags::bitflags;
use core::ops;

bitflags! {
	#[derive(Copy, Clone)]
	pub struct PTE: usize {
		const V = 1 << 0;
		const R = 1 << 1;
		const W = 1 << 2;
		const X = 1 << 3;
		const U = 1 << 4;
		const G = 1 << 5;
		const A = 1 << 6;
		const D = 1 << 7;
		const RW = 0b110;
		const RX = 0b1010;
	}
}

#[derive(Clone, Copy)]
pub struct PageTableEntry {
	pub bits: usize,
}

impl PageTableEntry {
	pub fn get_valid(&self) -> bool {
		self.bits & 1 != 0
	}
	pub fn get_ppn(&self) -> usize {
		self.bits >> 10 & ((1 << 44) - 1)
	}
	pub fn clear(&mut self) {
		self.bits = 0;
	}
	pub fn from_phys_addr(addr: usize) -> Self {
		Self {
			bits: (addr >> PAGE_SIZE_BITS) << 10,
		}
	}
	pub fn is_leaf(&self) -> bool {
		self.bits & 0b1110 != 0
	}
}

impl ops::BitOr<PTE> for PageTableEntry {
	type Output = Self;
	fn bitor(self, rhs: PTE) -> Self {
		Self {
			bits: self.bits | rhs.bits(),
		}
	}
}
impl ops::BitOr<usize> for PageTableEntry {
	type Output = Self;
	fn bitor(self, rhs: usize) -> Self {
		Self {
			bits: self.bits | rhs,
		}
	}
}
