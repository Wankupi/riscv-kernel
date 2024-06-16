use crate::{
	arch::regs,
	driver::uart::uart_device,
	mm::allocator::alloc_size_align,
	print::printk,
	shutdown,
	user::{copy_from_user, scheduler},
	IPC::msg::{sys_msg_recv, sys_msg_send},
};

use super::{scheduler::yield_this, task::Task};

use alloc::boxed::Box;
use SyscallAPI::SyscallID;

pub fn syscall(task: &mut Task) {
	let regs = &mut task.process.trapframe.as_mut().regs;
	regs.pc += 4;
	let syscall_id = SyscallID::from(regs.a7_x17);
	match syscall_id {
		SyscallID::DebugConsolePutchar => {
			uart_device.write(regs.a0_x10 as u8);
		}
		SyscallID::Exit => {
			log!("task exit with code {}", regs.a0_x10);
			scheduler::remove_task(task);
		}
		SyscallID::Write => sys_write(task),
		SyscallID::MsgSend => sys_msg_send(task),
		SyscallID::MsgRecv => sys_msg_recv(task),
		_ => {
			log!("unknown syscall id: {:?}", syscall_id);
			shutdown();
		}
	}
	yield_this();
}

fn sys_write(task: &mut Task) {
	let fd = task.process.trapframe.as_mut().regs.a0_x10;
	let src = task.process.trapframe.as_mut().regs.a1_x11 as *const u8;
	let len = task.process.trapframe.as_mut().regs.a2_x12;
	let mut buf = Box::<[u8; 4096]>::new([0; 4096]);
	let r = copy_from_user(buf.as_mut(), src, len, task);
	assert!(r == 0, "copy_from_user failed: {}", r);
	printk(core::str::from_utf8(buf.as_ref()).unwrap().as_bytes());
}
