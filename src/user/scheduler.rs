use alloc::{boxed::Box, vec::Vec};

use crate::{
	_switch,
	mm::vm::{kvm_map, PTE},
	PAGE_SIZE,
};

use super::{
	task::{Context, Task},
	trapframe::TrapFrame,
};

#[derive(Default)]
pub struct Scheduler {
	cur_list: Vec<Box<Task>>,
	next_list: Vec<Box<Task>>,
}

impl Scheduler {
	pub const fn new() -> Self {
		Scheduler {
			cur_list: Vec::new(),
			next_list: Vec::new(),
		}
	}
	pub fn add_task(&mut self, task: Box<Task>) {
		self.next_list.push(task);
	}
	pub fn remove_task(&mut self, task: &Task) {
		self.cur_list
			.retain(|t| !(t.as_ref() as *const Task == task as *const Task));
		self.next_list
			.retain(|t| !(t.as_ref() as *const Task == task as *const Task));
	}
	pub fn schedule(&mut self) -> Option<&mut Task> {
		if self.cur_list.is_empty() && self.next_list.is_empty() {
			return None;
		}
		if self.cur_list.is_empty() {
			self.cur_list.append(&mut self.next_list);
			self.cur_list.reverse();
		}
		let mut tail = self.cur_list.pop().unwrap();
		let ret = tail.as_mut() as *mut Task;
		self.next_list.push(tail);
		return Some(unsafe { &mut *ret });
	}
}
static mut scheduler: Scheduler = Scheduler::new();
static mut scheduler_context: Context = Context::new();

pub fn schedule_tasks() {
	loop {
		if let Some(task) = unsafe { scheduler.schedule() } {
			set_trampoline(task.process.trapframe.as_ref());
			unsafe {
				_switch(&mut scheduler_context, &mut task.context);
			}
		}
	}
}

pub fn yield_this(task: &mut Task) {
	unsafe {
		_switch(&mut task.context, &mut scheduler_context);
		hard_sleep();
	}
}

fn set_trampoline(task_trapframe: *const TrapFrame) {
	kvm_map(
		0xffffffff_ffff_e000,
		task_trapframe as usize,
		PAGE_SIZE,
		PTE::RW,
	);
}

pub fn add_task(task: Box<Task>) {
	unsafe {
		scheduler.add_task(task);
	}
}
pub fn remove_task(task: &Task) {
	unsafe {
		scheduler.remove_task(task);
	}
}

#[no_mangle]
extern "C" fn hard_sleep() {
	let mut x = 0;
	let p = &mut x as *mut i32;
	for _ in 0..5000000 {
		unsafe { p.write_volatile(10) }
	}
}
