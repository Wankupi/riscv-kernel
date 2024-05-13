use core::{num::Wrapping, panic};
use super::{PageTableEntry, VirtMapPage, PTE};
use crate::{asm_funcs::*, config::*, mm::allocator::simple_allocator};

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
pub fn get_kernel_satp() -> usize {
	unsafe { kvm_config.satp }
}

pub fn init_kvm() {
	let table_phys_addr = VirtMapPage::create();
	let k_vt = unsafe { &mut *table_phys_addr };
	unsafe {
		kvm_config.satp = k_vt.to_satp();
		kvm_config.v2p_offset_text = 0;
		// kvm_config.v2p_offset_text = 0xffff_ffff_0000_0000;
	}
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
	let simple_allocator_range = simple_allocator.get_control_range();
	vm_map(
		k_vt,
		simple_allocator_range.0,
		simple_allocator_range.0,
		simple_allocator_range.1 - simple_allocator_range.0,
		PTE::R | PTE::W,
	);
	log!("qqqaq");

	vm_map(
		k_vt,
		0xffffffff_ffff_f000,
		_trap_entry as usize,
		4096,
		PTE::R | PTE::X,
	);
	// direct map physical memory
	vm_map(
		k_vt,
		simple_allocator_range.1,
		simple_allocator_range.1,
		1024 * 1024 * 128,
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
		let p = VirtMapPage::create();
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

pub fn vm_map(vt: &mut VirtMapPage, va: usize, pa: usize, mut size: usize, flags: PTE) {
	if (va & 0xfff) != 0 || (pa & 0xfff) != 0 {
		panic!("vm_map: va or pa is not page aligned");
	}
	size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
	if (size & 0xfff) != 0 {
		log!("vm_map: size is not page aligned: {:x}", size);
	}
	let mut va = Wrapping(va);
	let mut pa = Wrapping(pa);
	let mut size = Wrapping(size);
	info!(
		"vm_map: va=[{:x}, {:x})  pa=[{:x}, {:x})  size={:x}, flags={:x}",
		va,
		va + size,
		pa,
		pa + size,
		size,
		flags
	);
	while size.0 >= PTE_CONTROL_SIZE_0 && (va.0 & (PTE_CONTROL_SIZE_1 - 1)) != 0 {
		vm_level0(vt, va.0, pa.0, flags);
		va += PTE_CONTROL_SIZE_0;
		pa += PTE_CONTROL_SIZE_0;
		size -= PTE_CONTROL_SIZE_0;
	}
	while size.0 >= PTE_CONTROL_SIZE_1 && (va.0 & (PTE_CONTROL_SIZE_2 - 1)) != 0 {
		vm_level1(vt, va.0, pa.0, flags);
		va += PTE_CONTROL_SIZE_1;
		pa += PTE_CONTROL_SIZE_1;
		size -= PTE_CONTROL_SIZE_1;
	}
	while size.0 >= PTE_CONTROL_SIZE_2 {
		vm_level2(vt, va.0, pa.0, flags);
		va += PTE_CONTROL_SIZE_2;
		pa += PTE_CONTROL_SIZE_2;
		size -= PTE_CONTROL_SIZE_2;
	}
	while size.0 >= PTE_CONTROL_SIZE_1 {
		vm_level1(vt, va.0, pa.0, flags);
		va += PTE_CONTROL_SIZE_1;
		pa += PTE_CONTROL_SIZE_1;
		size -= PTE_CONTROL_SIZE_1;
	}
	while size.0 >= PTE_CONTROL_SIZE_0 {
		vm_level0(vt, va.0, pa.0, flags);
		va += PTE_CONTROL_SIZE_0;
		pa += PTE_CONTROL_SIZE_0;
		size -= PTE_CONTROL_SIZE_0;
	}
	// log!("done");
}

pub fn vm_map_trampoline(vt: &mut VirtMapPage) {
	let kvt = get_kernel_vtable();
	assert!(kvt.entries[511].bits != 0);
	vt.entries[511] = kvt.entries[511];
}
