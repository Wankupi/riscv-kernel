use core::{arch::asm, panic};

use crate::{arch::mm::*, success};

#[repr(align(4096))]
pub struct VirtMapPage {
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
}

use alloc::alloc::{alloc, Layout};
static mut k_vt_ptr: *mut VirtMapPage = 0 as *mut VirtMapPage;

extern "C" {
	fn stext();
	fn etext();
	fn srodata();
	fn erodata();
	fn sdata();
	fn edata();
	fn sbss();
	fn ebss();
	fn uart_base_addr();
	fn system_reset_addr();
}

pub fn init_kvm() {
	unsafe {
		k_vt_ptr = VirtMapPage::create();
	}
	let k_vt = unsafe { &mut *k_vt_ptr };
	// kernel source code
	kvm_map(k_vt, stext as usize, etext as usize, PTE::R | PTE::X);
	// kernel rodata
	kvm_map(k_vt, srodata as usize, erodata as usize, PTE::R);
	// kernel data
	kvm_map(k_vt, sdata as usize, edata as usize, PTE::R | PTE::W);
	// kernel bss
	kvm_map(k_vt, sbss as usize, ebss as usize, PTE::R | PTE::W);
	// uart device
	kvm_map(
		k_vt,
		uart_base_addr as usize,
		uart_base_addr as usize,
		PTE::R | PTE::W | PTE::X,
	);
	// shutdown io
	kvm_map(
		k_vt,
		system_reset_addr as usize,
		system_reset_addr as usize,
		PTE::R | PTE::W,
	);
	success!("create virtual table");
}

pub fn kvm_start() {
	let satp = unsafe { (8 << 60) | (k_vt_ptr as usize >> 12) };
	unsafe {
		asm!("sfence.vma");
		asm!("csrw satp, {}", in(reg) satp);
	}
	success!("set satp");
}

fn entry_to_next_table(entry: &PageTableEntry) -> &mut VirtMapPage {
	unsafe { &mut *((entry.get_ppn() << PAGE_SIZE_BITS) as *mut VirtMapPage) }
}

fn kvm_get_entry(_vt: &mut VirtMapPage, va: usize, is_create: bool) -> &mut PageTableEntry {
	let idx = [
		(va >> (12)) & 0x1ff,
		(va >> (9 + 12)) & 0x1ff,
		(va >> (9 + 9 + 12)) & 0x1ff,
	];
	let mut vt = _vt;
	for i in (1..=2).rev() {
		let k = idx[i];
		let entry = &mut vt.entries[k];
		if !entry.get_valid() {
			if !is_create {
				panic!("not support");
			}
			let p = unsafe { VirtMapPage::create() };
			*entry = PageTableEntry::from_phys_addr(p as usize) | PTE::V;
		}
		vt = entry_to_next_table(entry);
	}
	return &mut vt.entries[idx[0]];
}

fn kvm_map(vt: &mut VirtMapPage, start: usize, end: usize, flags: PTE) {
	vm_map(vt, start, start, end - start + 1, flags);
}

fn vm_map(vt: &mut VirtMapPage, va: usize, pa: usize, size: usize, flags: PTE) {
	let mut v = va & !(PAGE_SIZE - 1);
	let end = (va + size - 1) & !(PAGE_SIZE - 1);
	let mut p = pa & !(PAGE_SIZE - 1);
	loop {
		let entry = kvm_get_entry(vt, v, true);
		*entry = PageTableEntry::from_phys_addr(p) | flags | PTE::V;
		if v == end {
			break;
		}
		v += PAGE_SIZE;
		p += PAGE_SIZE;
	}
}
