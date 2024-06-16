use core::arch::asm;

use alloc::{boxed::Box, vec::Vec};

use crate::{
	_switch,
	mm::vm::{kvm_map, PTE},
	print::printk,
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
	pub fn remove_task(&mut self, task: &Task) -> Box<Task> {
		let ret;
		let filter = |t: &Box<Task>| t.as_ref() as *const Task == task as *const Task;
		if let Some(idx) = self.cur_list.iter().position(filter) {
			ret = self.cur_list.remove(idx);
		} else if let Some(idx) = self.next_list.iter().position(filter) {
			ret = self.next_list.remove(idx);
		} else {
			panic!("task not found");
		}
		ret
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
	let mut times = 0;
	loop {
		if let Some(task) = unsafe { scheduler.schedule() } {
			set_trampoline(task.process.trapframe.as_ref());
			unsafe { _switch(&mut scheduler_context, &mut task.context) };
			times = 0;
		} else {
			println!("no task to run");
			times += 1;
			for _ in 0..(times * times) {
				hard_sleep();
			}
		}
	}
}

pub fn yield_this() {
	let trampoline = unsafe { &mut *(0xffffffff_ffff_e000 as *mut TrapFrame) };
	let task = unsafe { &mut *trampoline.task.unwrap() };
	unsafe {
		_switch(&mut task.context, &mut scheduler_context);
		hard_sleep();
	}
}

fn set_trampoline(task_trapframe: *const TrapFrame) {
	let addr = 0xffffffff_ffff_e000;
	kvm_map(addr, task_trapframe as usize, PAGE_SIZE, PTE::RW);
	unsafe { asm!("sfence.vma {}, zero", in(reg) addr) };
}

pub fn add_task(task: Box<Task>) {
	unsafe {
		scheduler.add_task(task);
	}
}
pub fn remove_task(task: &Task) -> Box<Task> {
	unsafe { scheduler.remove_task(task) }
}

#[no_mangle]
extern "C" fn hard_sleep() {
	let mut x = 0;
	let p = &mut x as *mut i32;
	for _ in 0..50000000 {
		unsafe { p.write_volatile(10) }
	}
}
