use crate::{
	arch::regs,
	driver::uart::uart_device,
	mm::allocator::alloc_size_align,
	print::{self, printk},
	shutdown,
	user::{copy_from_user, copy_to_user, get_userapp_by_name, scheduler},
	IPC::msg::{sys_msg_recv, sys_msg_send},
};

use super::{process::PidSet, scheduler::yield_this, task::Task};

use alloc::{
	boxed::Box,
	collections::{BTreeMap, BTreeSet},
	vec::Vec,
};
use SyscallAPI::SyscallID;

pub fn syscall(task: &mut Task) {
	let regs = &mut task.process.trapframe.as_mut().regs;
	regs.pc += 4;
	let syscall_id = SyscallID::from(regs.a7_x17);
	match syscall_id {
		SyscallID::DebugConsolePutchar => {
			unsafe { uart_device.write(regs.a0_x10 as u8) };
		}
		SyscallID::Read => sys_read(task),
		SyscallID::Exit => sys_exit(task),
		SyscallID::ForkExec => sys_forkexec(task),
		SyscallID::Write => sys_write(task),
		SyscallID::MsgSend => sys_msg_send(task),
		SyscallID::MsgRecv => sys_msg_recv(task),
		SyscallID::Wait => sys_wait(task),
		_ => {
			log!("unknown syscall id: {:?}", syscall_id);
			shutdown();
		}
	}
	yield_this();
}

fn sys_read(task: &mut Task) {
	// print!("r");
	let fd = task.process.trapframe.as_mut().regs.a0_x10;
	let src = task.process.trapframe.as_mut().regs.a1_x11 as *mut u8;
	let len = task.process.trapframe.as_mut().regs.a2_x12;
	assert!(fd == 0);
	assert!(len >= 1);
	let mut b = unsafe { uart_device.read() };
	match b {
		13 => {
			b = b'\n';
		}
		_ => {}
	}
	let buf = &[b];
	// println!("read: {:?}", buf);
	assert!(copy_to_user(src, buf, 1, task) == 0, "coyp_to_user failed");
	task.process.trapframe.as_mut().regs.a0_x10 = 1;
}

fn sys_write(task: &mut Task) {
	let fd = task.process.trapframe.as_mut().regs.a0_x10;
	let src = task.process.trapframe.as_mut().regs.a1_x11 as *const u8;
	let len = task.process.trapframe.as_mut().regs.a2_x12;
	let mut buf = Box::<[u8; 4096]>::new([0; 4096]);
	let r = copy_from_user(buf.as_mut(), src, len, task);
	assert!(r == 0, "copy_from_user failed: {}", r);
	assert!(fd == 1, "fd != 1");
	printk(core::str::from_utf8(buf.as_ref()).unwrap().as_bytes());
}

static mut wait_list: BTreeMap<usize, Vec<Box<Task>>> = BTreeMap::new();

fn sys_wait(task: &mut Task) {
	let pid = task.process.trapframe.as_mut().regs.a0_x10;
	if !unsafe { PidSet.contains(&pid) } {
		task.process.trapframe.as_mut().regs.a0_x10 = !0usize;
		return;
	}
	if !unsafe { wait_list.contains_key(&pid) } {
		unsafe { wait_list.insert(pid, Vec::new()) };
	}
	let list = unsafe { wait_list.get_mut(&pid).unwrap() };
	let task_box = scheduler::remove_task(task);
	list.push(task_box);
	yield_this();
}

fn sys_exit(task: &mut Task) {
	let code = task.process.trapframe.as_mut().regs.a0_x10;
	// log!("task exit with code {}", code);
	let task_box = scheduler::remove_task(task);
	let pid = task.process.pid;
	unsafe { PidSet.remove(&pid) };
	let list = unsafe { wait_list.get_mut(&pid).unwrap() };
	while !list.is_empty() {
		let mut wait_task_box = list.pop().unwrap();
		wait_task_box.process.trapframe.regs.a0_x10 = code;
		scheduler::add_task(wait_task_box);
	}
	unsafe { wait_list.remove(&pid) };
	// finish the all thing of this task
	scheduler::add_died_task(task_box);
	yield_this();
}

fn sys_forkexec(task: &mut Task) {
	let name_addr = task.process.trapframe.as_mut().regs.a0_x10 as *const u8;
	let name_len = task.process.trapframe.as_mut().regs.a1_x11;
	let mut name = Vec::<u8>::with_capacity(name_len);
	name.resize(name_len, 0);
	assert!(
		copy_from_user(name.as_mut(), name_addr, name_len, task) == 0,
		"copy_from_user failed"
	);
	println!("forkexec: {:?}", core::str::from_utf8(&name).unwrap());
	let data = get_userapp_by_name(core::str::from_utf8(&name).unwrap());
	match data {
		None => {
			println!("no such file: {:?}", core::str::from_utf8(&name).unwrap());
			task.process.trapframe.as_mut().regs.a0_x10 = !0usize;
			return;
		}
		_ => {}
	}
	let data = data.unwrap();
	let sub_task = Task::from_elf(data);
	task.process.trapframe.as_mut().regs.a0_x10 = sub_task.process.pid as usize;
	scheduler::add_task(sub_task);
}
