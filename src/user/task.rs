// use core::default;

use core::{borrow::BorrowMut, cell::RefCell};

use alloc::{borrow::ToOwned, boxed::Box, rc::Rc, sync::Arc};

use super::trapframe::TrapFrame;
use crate::mm::vm::{vm_map, vm_map_trampoline, VirtMapPage};

#[derive(Default)]
enum TaskState {
	#[default]
	Ready,
	Running,
	Sleeping,
	Stopped,
	Zombie,
}

#[derive(Default, Clone, Copy)]
pub struct Section {
	pub vaddr: usize,
	pub paddr: usize,
	pub size: usize,
}

#[derive(Default)]
pub struct Sections {
	// [start, end): left close and right open
	pub text: Section,
	pub data: Section,
	pub heap: Section,
	pub stack: Section,
}

pub struct ProcessResource {
	pub page_table: Box<VirtMapPage>,
}

#[derive(Default)]
pub struct Task {
	pid: i32,
	tid: i32,
	uid: i32,
	parent_pid: i32,
	state: TaskState,
	trap_frame: Box<TrapFrame>,
	name: [u8; 16],
	// resource: Arc<RefCell<ProcessResource>>,
	segments: Sections,
	// TODO: files
}

impl Task {
	pub fn new_box() -> Box<Self> {
		Box::new(Self::default())
	}
}

impl Task {
	// pub fn init_seg(&mut self) {
	// 	let seg = &mut self.segments;
	// 	seg.stack.1 = 0xeeeeeeee_00000000;
	// 	seg.stack.0 = seg.stack.1 - 0x1000;
	// 	seg.heap.0 = 0x01000000_00000000;
	// 	seg.heap.1 = seg.heap.0;
	// }
	pub fn init_registers(&mut self) {
		let tf = self.trap_frame.as_mut();
		let regs = &mut tf.regs;
		// ensure main return to trap entry
		regs.ra_x1 = 0xffffffff_ffff_f000;
		regs.sp_x2 = self.segments.stack.vaddr;
	}
	// pub fn build_pagetable(&mut self) {
	// 	let vt = VirtMapPage::create_ref();
	// 	vm_map_trampoline(vt);
	// 	let seg = &self.segments;
	// }
}
