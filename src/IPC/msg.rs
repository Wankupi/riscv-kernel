use alloc::{
	boxed::Box,
	collections::{BTreeMap, VecDeque},
	vec::Vec,
};

use crate::user::{
	copy_from_user, copy_to_user,
	scheduler::{self, yield_this},
	task::Task,
};

#[derive(Debug)]
enum MsgType {
	Recv,
	Send,
}

pub struct Msg {
	task: Box<Task>,
	len: usize,
	uaddr: *mut u8,
}

struct MsgQueue {
	recv: VecDeque<Msg>,
	send: VecDeque<Msg>,
}

pub type MsgQueueKey = usize;
static mut MsgQueueMap: *mut BTreeMap<MsgQueueKey, MsgQueue> = 0 as *mut _;

pub fn init() {
	unsafe { MsgQueueMap = Box::into_raw(Box::new(BTreeMap::new())) };
}

fn get_map() -> &'static mut BTreeMap<MsgQueueKey, MsgQueue> {
	unsafe { &mut *MsgQueueMap }
}

pub fn find_or_create_msg_queue(key: MsgQueueKey) -> &'static mut MsgQueue {
	let map = get_map();
	if !map.contains_key(&key) {
		map.insert(
			key,
			MsgQueue {
				recv: VecDeque::new(),
				send: VecDeque::new(),
			},
		);
	}
	map.get_mut(&key).unwrap()
}

pub fn msg_send(key: MsgQueueKey, task: &mut Task, len: usize, uaddr: *mut u8) {
	let queue = find_or_create_msg_queue(key);
	loop {
		if queue.recv.is_empty() {
			let box_task = scheduler::remove_task(task);
			queue.send.push_back(Msg {
				task: box_task,
				len,
				uaddr,
			});
			yield_this(); // release when queue.recv is not empty
			  // the task will be put back to scheduler by msg_recv
		}
		if queue.recv.front_mut().unwrap().len < len {
			let mut recv_task = queue.recv.pop_front().unwrap().task;
			recv_task.get_regs().a0_x10 = (-(len as isize)) as usize;
			scheduler::add_task(recv_task);
			continue;
		}
		let recv_msg = queue.recv.front_mut().unwrap();
		let mut buf = Vec::<u8>::with_capacity(len);
		buf.resize(len, 0);
		let arr = buf.as_mut_slice();
		copy_from_user(arr, uaddr, len, task);
		copy_to_user(recv_msg.uaddr, arr, len, recv_msg.task.as_mut());

		let mut recv_task = queue.recv.pop_front().unwrap().task;
		recv_task.get_regs().a0_x10 = len;
		scheduler::add_task(recv_task);
		break;
	}
	task.get_regs().a0_x10 = len;
}

pub fn msg_recv(key: MsgQueueKey, task: &mut Task, maxlen: usize, uaddr: *mut u8) {
	let queue = find_or_create_msg_queue(key);
	let box_task = scheduler::remove_task(task);
	queue.recv.push_back(Msg {
		task: box_task,
		len: maxlen,
		uaddr,
	});
	if !queue.send.is_empty() {
		let send_msg = queue.send.pop_front().unwrap();
		let send_task = send_msg.task;
		scheduler::add_task(send_task);
	}
	yield_this();
	// return val is set by msg_send
}

pub fn sys_msg_send(task: &mut Task) {
	let key = task.process.trapframe.as_mut().regs.a0_x10;
	let uaddr = task.process.trapframe.as_mut().regs.a1_x11 as *mut u8;
	let len = task.process.trapframe.as_mut().regs.a2_x12;
	msg_send(key, task, len, uaddr);
}

pub fn sys_msg_recv(task: &mut Task) {
	let key = task.process.trapframe.as_mut().regs.a0_x10;
	let uaddr = task.process.trapframe.as_mut().regs.a1_x11 as *mut u8;
	let len = task.process.trapframe.as_mut().regs.a2_x12;
	msg_recv(key, task, len, uaddr);
}
