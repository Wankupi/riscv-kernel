use core::{alloc::Layout, mem::size_of};

use alloc::{boxed::Box, vec::Vec};
use xmas_elf::{
	program::{ProgramHeader, SegmentData},
	ElfFile,
};

use crate::{
	_trap_entry, alloc, lang::memcpy, mm::vm::{get_kernel_satp, vm_map, vm_map_trampoline, VirtMapPage, PTE}, uart_base_addr, PAGE_SIZE
};

use super::trapframe::TrapFrame;

#[derive(Default, Clone, Copy)]
pub struct Section {
	pub vaddr: usize,
	pub paddr: usize,
	pub size: usize,
}

#[derive(Default)]
pub struct Sections {
	pub heap: Section,
	pub stack: Section,
}

pub struct KernelStack {
	pub data: [u8; PAGE_SIZE],
}
impl KernelStack {
	pub fn new_box() -> Box<Self> {
		Box::new(Self {
			data: [0; PAGE_SIZE],
		})
	}
}

pub struct Process {
	// pub name: [u8; 16],
	pub pagetable: Box<VirtMapPage>,
	pub static_data: Vec<Section>,
	pub trapframe: Box<TrapFrame>,
	pub kernel_stack: Box<KernelStack>,
	pub stack: usize,
	pub heap: usize,
}

impl Process {
	pub fn init_registers(&mut self, elf: &ElfFile) {
		let tf = self.trapframe.as_mut();
		let regs = &mut tf.regs;
		regs.sp_x2 = self.stack;
		// ? ensure main return to trap entry
		// regs.ra_x1 = 0xffffffff_ffff_f000;
		regs.pc = elf.header.pt2.entry_point() as usize;
	}
}
pub fn create_process(elf_data: &[u8]) -> Result<Box<Process>, &'static str> {
	let elf = ElfFile::new(elf_data).unwrap();
	if elf.header.pt1.magic != [0x7f, 0x45, 0x4c, 0x46] {
		return Err("elf magic number not correct");
	}
	let (page_table, static_data) = create_pagetable_from_elf(&elf);
	let mut process = Process {
		pagetable: page_table,
		static_data,
		trapframe: TrapFrame::new_box(),
		kernel_stack: KernelStack::new_box(),
		stack: 1 << 38,
		heap: 0x1_00000000,
	};
	process.init_registers(&elf);
	let stack = alloc(Layout::from_size_align(PAGE_SIZE, PAGE_SIZE).unwrap());
	vm_map(
		&mut process.pagetable,
		process.stack - PAGE_SIZE,
		stack as usize,
		PAGE_SIZE,
		PTE::RW | PTE::U,
	);
	let tf = process.trapframe.as_mut();
	tf.kernel_satp = get_kernel_satp();
	tf.kernel_sp = process.kernel_stack.data.as_ptr() as usize + size_of::<KernelStack>();
	tf.kernel_trap = _trap_entry as usize;
	tf.hartid = 0;
	tf.satp = process.pagetable.to_satp();
	println!("addr of tf: {:x}", tf as *const _ as usize);
	println!("satp: {:x} {:x}", tf.satp, &tf.satp as *const _ as usize);
	return Ok(Box::new(process));
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
