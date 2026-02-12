use core::mem::size_of;

use alloc::boxed::Box;

use crate::arch::{regs::Registers, trap::run_user};

use super::{
	process::{self, KernelStack, Process},
	trapframe::TrapFrame,
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
#[repr(C)]
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
		Self {
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
	// remember how long the task has run
	// important for scheduler to know take which method to run this thread
	// pub cputime: usize,
}

impl Task {
	pub fn new(process: Box<Process>) -> Self {
		Self {
			state: TaskState::Ready,
			process,
			context: Context::default(),
			// cputime: 0,
		}
	}
	pub fn new_box(process: Box<Process>) -> Box<Self> {
		Box::new(Self::new(process))
	}
	pub fn from_elf(elf_data: &[u8]) -> Box<Task> {
		let process: Box<Process> = process::create_process(elf_data).unwrap();
		let mut task = Task::new_box(process);
		task.process.trapframe.task = Some(task.as_mut() as *mut Task as *mut Task);
		let context = &mut task.context;
		context.ra = run_user as usize;
		context.sp =
			task.process.kernel_stack.as_ref() as *const _ as usize + size_of::<KernelStack>();
		task
	}
	pub fn get_tramframe(&mut self) -> *mut TrapFrame {
		self.process.trapframe.as_ref() as *const TrapFrame as *mut TrapFrame
	}
	pub fn get_regs(&mut self) -> &mut Registers {
		&mut self.process.trapframe.as_mut().regs
	}
}
