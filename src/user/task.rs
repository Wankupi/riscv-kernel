use alloc::{borrow::ToOwned, boxed::Box};
use xmas_elf::ElfFile;

use super::{
	process::{self, Process},
	trapframe::TrapFrame,
};
use crate::{
	mm::vm::{vm_map, vm_map_trampoline, VirtMapPage},
	PAGE_SIZE,
};

#[derive(Default)]
enum TaskState {
	#[default]
	Ready,
	Running,
	Sleeping,
	Stopped,
	Zombie,
}

#[derive(Default)]
pub struct Context {
	ra: usize,
	sp: usize,
	s0: usize,
	s1: usize,
	s2: usize,
	s3: usize,
	s4: usize,
	s5: usize,
	s6: usize,
	s7: usize,
	s8: usize,
	s9: usize,
	s10: usize,
	s11: usize,
}

impl Context {
	pub const fn new() -> Self {
		Context {
			ra: 0,
			sp: 0,
			s0: 0,
			s1: 0,
			s2: 0,
			s3: 0,
			s4: 0,
			s5: 0,
			s6: 0,
			s7: 0,
			s8: 0,
			s9: 0,
			s10: 0,
			s11: 0,
		}
	}
}

pub struct Task {
	pub state: TaskState,
	pub process: Box<Process>,
	pub context: Context,
}

impl Task {
	// pub fn init_seg(&mut self) {
	// 	let seg = &mut self.segments;
	// 	seg.stack.1 = 0xeeeeeeee_00000000;
	// 	seg.stack.0 = seg.stack.1 - 0x1000;
	// 	seg.heap.0 = 0x01000000_00000000;
	// 	seg.heap.1 = seg.heap.0;
	// }

	// pub fn build_pagetable(&mut self) {
	// 	let vt = VirtMapPage::create_ref();
	// 	vm_map_trampoline(vt);
	// 	let seg = &self.segments;
	// }
}

impl Task {
	pub fn new(process: Box<Process>) -> Self {
		let task = Task {
			state: TaskState::Ready,
			process: process,
			context: Context::default(),
		};
		task
	}
	pub fn new_box(process: Box<Process>) -> Box<Self> {
		Box::new(Self::new(process))
	}
	pub fn from_elf(elf_data: &[u8]) -> Box<Task> {
		let process = process::create_process(elf_data).unwrap();
		Task::new_box(process)
	}
}
