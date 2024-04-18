use crate::{alloc, arch::mm::*};
use alloc::alloc::Layout;
use core::panic;

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

pub struct KernelVirtMapConfig {
	// pub table_phys_addr: *mut VirtMapPage,
	pub satp: usize,
	pub v2p_offset_text: usize,
}
#[no_mangle]
pub static mut kvm_config: KernelVirtMapConfig = KernelVirtMapConfig {
	v2p_offset_text: 0,
	satp: 0,
};

pub fn get_kernel_v2p_offset() -> usize {
	unsafe { kvm_config.v2p_offset_text }
}
pub fn get_kernel_vtable() -> &'static mut VirtMapPage {
	unsafe { &mut *(((kvm_config.satp & ((1 << 44) - 1)) << 12) as *mut VirtMapPage) }
}

extern "C" {
	fn stext();
	fn etext();
	fn srodata();
	fn erodata();
	fn sdata();
	fn edata();
	fn sbss();
	fn ebss();
	// fn uart_base_addr();
	// fn system_reset_addr();
	fn skernel();
	fn ekernel();
}
use crate::config::*;

pub fn init_kvm() {
	let table_phys_addr = unsafe { VirtMapPage::create() };
	unsafe {
		kvm_config.satp = (table_phys_addr as usize >> PAGE_SIZE_BITS) | (8 << 60);
		kvm_config.v2p_offset_text = 0;
		// kvm_config.v2p_offset_text = 0xffff_ffff_0000_0000;
	}
	let k_vt = unsafe { &mut *table_phys_addr };
	// kernel source code
	kvm_map_early(stext as usize, etext as usize, PTE::R | PTE::X);
	// kernel rodata
	kvm_map_early(srodata as usize, erodata as usize, PTE::R);
	// kernel data
	kvm_map_early(sdata as usize, edata as usize, PTE::R | PTE::W);
	// kernel bss ( with stack )
	kvm_map_early(sbss as usize, ebss as usize, PTE::R | PTE::W);
	// uart device
	vm_map(
		k_vt,
		uart_base_addr as usize,
		uart_base_addr as usize,
		PAGE_SIZE,
		PTE::R | PTE::W | PTE::X,
	);
	// shutdown io
	vm_map(
		k_vt,
		system_reset_addr as usize,
		system_reset_addr as usize,
		PAGE_SIZE,
		PTE::R | PTE::W,
	);
	let simple_allocator_range = super::simple_allocator.get_control_range();
	vm_map(
		k_vt,
		simple_allocator_range.0,
		simple_allocator_range.0,
		simple_allocator_range.1 - simple_allocator_range.0,
		PTE::R | PTE::W,
	);
}

fn entry_to_next_table(entry: &PageTableEntry) -> &mut VirtMapPage {
	unsafe { &mut *((entry.get_ppn() << PAGE_SIZE_BITS) as *mut VirtMapPage) }
}

fn vt_get_entry(vt: &mut VirtMapPage, vindex: usize) -> &mut PageTableEntry {
	&mut vt.entries[vindex & (1 << 9) - 1]
}
fn vt_next_level(vt: &mut VirtMapPage, vindex: usize) -> &mut VirtMapPage {
	let entry = vt_get_entry(vt, vindex);
	if !entry.get_valid() {
		let p = unsafe { VirtMapPage::create() };
		*entry = PageTableEntry::from_phys_addr(p as usize) | PTE::V;
	} else if entry.is_leaf() {
		panic!("vt_next_level: entry is leaf");
	}
	let addr = entry.get_ppn() << PAGE_SIZE_BITS;
	unsafe { &mut *(addr as *mut VirtMapPage) }
}

fn vm_level2(vt: &mut VirtMapPage, va: usize, pa: usize, flags: PTE) {
	let entry = vt_get_entry(vt, va >> (12 + 9 + 9) & 0x1ff);
	*entry = PageTableEntry::from_phys_addr(pa) | flags | PTE::V;
}
fn vm_level1(mut vt: &mut VirtMapPage, va: usize, pa: usize, flags: PTE) {
	vt = vt_next_level(vt, va >> (12 + 9 + 9) & 0x1ff);
	let entry = vt_get_entry(vt, va >> (12 + 9) & 0x1ff);
	*entry = PageTableEntry::from_phys_addr(pa) | flags | PTE::V;
}
fn vm_level0(mut vt: &mut VirtMapPage, va: usize, pa: usize, flags: PTE) {
	vt = vt_next_level(vt, (va >> (12 + 9 + 9)) & 0x1ff);
	vt = vt_next_level(vt, (va >> (12 + 9)) & 0x1ff);
	let entry = vt_get_entry(vt, (va >> 12) & 0x1ff);
	*entry = PageTableEntry::from_phys_addr(pa) | flags | PTE::V;
}

/// @param [start, end)
pub fn kvm_map_early(start: usize, end: usize, flags: PTE) {
	let offset = unsafe { kvm_config.v2p_offset_text };
	kvm_map(start + offset, start, end - start, flags);
}
pub fn kvm_map(va: usize, pa: usize, size: usize, flags: PTE) {
	let vt = get_kernel_vtable();
	vm_map(vt, va, pa, size, flags);
}

const PTE_CONTROL_SIZE_0: usize = PAGE_SIZE;
const PTE_CONTROL_SIZE_1: usize = PTE_CONTROL_SIZE_0 << 9;
const PTE_CONTROL_SIZE_2: usize = PTE_CONTROL_SIZE_1 << 9;

pub fn vm_map(vt: &mut VirtMapPage, mut va: usize, mut pa: usize, mut size: usize, flags: PTE) {
	if (va & 0xfff) != 0 || (pa & 0xfff) != 0 {
		panic!("vm_map: va or pa is not page aligned");
	}
	size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
	if (size & 0xfff) != 0 {
		log!("vm_map: size is not page aligned: {:x}", size);
	}
	info!(
		"vm_map: va=[{:x}, {:x})  pa=[{:x}, {:x})  size={:x}, flags={:x}",
		va, va + size, pa, pa + size, size, flags
	);
	while size >= PTE_CONTROL_SIZE_0  && (va & (PTE_CONTROL_SIZE_1 - 1)) != 0 {
		vm_level0(vt, va, pa, flags);
		va += PTE_CONTROL_SIZE_0;
		pa += PTE_CONTROL_SIZE_0;
		size -= PTE_CONTROL_SIZE_0;
	}
	while size >= PTE_CONTROL_SIZE_1 && (va & (PTE_CONTROL_SIZE_2 - 1)) != 0 {
		vm_level1(vt, va, pa, flags);
		va += PTE_CONTROL_SIZE_1;
		pa += PTE_CONTROL_SIZE_1;
		size -= PTE_CONTROL_SIZE_1;
	}
	while size >= PTE_CONTROL_SIZE_2 {
		vm_level2(vt, va, pa, flags);
		va += PTE_CONTROL_SIZE_2;
		pa += PTE_CONTROL_SIZE_2;
		size -= PTE_CONTROL_SIZE_2;
	}
	while size >= PTE_CONTROL_SIZE_1 {
		vm_level1(vt, va, pa, flags);
		va += PTE_CONTROL_SIZE_1;
		pa += PTE_CONTROL_SIZE_1;
		size -= PTE_CONTROL_SIZE_1;
	}
	while size >= PTE_CONTROL_SIZE_0 {
		vm_level0(vt, va, pa, flags);
		va += PTE_CONTROL_SIZE_0;
		pa += PTE_CONTROL_SIZE_0;
		size -= PTE_CONTROL_SIZE_0;
	}
}
