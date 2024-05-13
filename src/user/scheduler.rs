use alloc::{boxed::Box, vec::Vec};

use crate::_switch;

use super::task::{Context, Task};

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
			self.next_list.reverse();
		}
		let mut tail = self.cur_list.pop().unwrap();
		let ret = tail.as_mut() as *mut Task;
		self.next_list.push(tail);
		return Some(unsafe { &mut *ret });
	}
}
static mut scheduler: Scheduler = Scheduler::new();
static mut scheduler_context: Context = Context::new();
fn schedule_tasks() {
	loop {
		if let Some(task) = unsafe { scheduler.schedule() } {
			unsafe {
				_switch(&mut scheduler_context, &mut task.context);
			}
		}
	}
}
