use crate::{driver::uart::uart_device, shutdown};

use super::{scheduler::yield_this, task::Task};

use SyscallAPI::SyscallID;

pub fn syscall(task: &mut Task) {
	let regs = &mut task.process.trapframe.regs;
	let syscall_id = SyscallID::from(regs.a7_x17);
	match syscall_id {
		SyscallID::DebugConsolePutchar => {
			uart_device.write(regs.a0_x10 as u8);
		}
		_ => {
			log!("unknown syscall id: {:?}", syscall_id);
			shutdown();
		}
	}
	regs.pc += 4;
	yield_this(task);
}
