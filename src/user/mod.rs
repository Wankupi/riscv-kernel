use core::alloc::Layout;

use alloc::{boxed::Box, vec::Vec};
use xmas_elf::{
	program::{ProgramHeader, SegmentData},
	ElfFile,
};

use crate::{
	lang::memcpy,
	mm::vm::{vm_map, vm_map_trampoline, VirtMapPage, PTE},
	uart_base_addr, PAGE_SIZE,
};

use self::task::{ProcessResource, Section, Sections};

pub mod task;
pub mod trapframe;

fn create_process(elf_data: &[u8]) -> Result<Box<ProcessResource>, &'static str> {
	let elf = ElfFile::new(elf_data).unwrap();

	if elf.header.pt1.magic != [0x7f, 0x45, 0x4c, 0x46] {
		return Err("elf magic number not correct");
	}

	return Err("function not done");
}

fn create_pagetable_from_elf(elf: &ElfFile) -> (Box<VirtMapPage>, Vec<Section>) {
	let mut vt_box = VirtMapPage::create_box();
	let vt = vt_box.as_mut();
	vm_map_trampoline(vt);
	let mut sections = Vec::<Section>::new();
	for header in elf.program_iter() {
		if header.get_type().unwrap() != xmas_elf::program::Type::Load {
			continue;
		}
		let mut pte = PTE::U;
		let flags = header.flags();
		if flags.is_read() {
			pte |= PTE::R;
		}
		if flags.is_write() {
			pte |= PTE::W;
		}
		if flags.is_execute() {
			pte |= PTE::X;
		}
		let section = create_phys_and_copy(&header, elf);
		vm_map(vt, section.vaddr, section.paddr, section.size, pte);
		sections.push(section);
	}
	// special
	vm_map(vt, uart_base_addr, uart_base_addr, 4096, PTE::RW | PTE::U);
	return (vt_box, sections);
}

fn create_phys_and_copy(header: &ProgramHeader, elf: &ElfFile) -> Section {
	let start = header.virtual_addr() as usize;
	let size = header.mem_size() as usize;
	let vstart = start & !(PAGE_SIZE - 1);
	let vend = (start + size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
	let offset = start - vstart;
	let layout = Layout::from_size_align(vend - vstart, PAGE_SIZE).unwrap();
	let pstart = crate::mm::alloc(layout);
	let data: &[u8] = if let SegmentData::Undefined(data) = header.get_data(elf).unwrap() {
		data
	} else {
		panic!("segment data not found");
	};
	memcpy(pstart.wrapping_add(offset), data.as_ptr(), size);
	return Section {
		vaddr: vstart,
		paddr: pstart as usize,
		size: vend - vstart,
	};
}
