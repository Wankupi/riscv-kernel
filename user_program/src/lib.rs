#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

use core::{panic::PanicInfo, usize};

extern "C" {
	fn main(a0: usize, a1: usize, a2: usize) -> isize;
}

#[no_mangle]
pub extern "C" fn _start(a0: usize, a1: usize, a2: usize) -> isize {
	unsafe { main(a0, a1, a2) }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

pub mod syscall {
	use core::arch::asm;

	enum SyscallID {
		Fork = 57,
		DebugConsoleWrite = 512,
		DebugConsolePutchar = 513,
	}
	// calling convention for syscalls
	// a7: syscall number
	// a0-a6: arguments
	fn syscall1(syscall_id: SyscallID, args1: usize) {
		unsafe {
			asm!("ecall", in("a7") syscall_id as usize, in("a0") args1);
		}
	}

	pub fn debug_console_write(s: &str) {
		syscall1(SyscallID::DebugConsoleWrite, s.as_ptr() as usize);
	}
	pub fn debug_console_putchar(c: u8) {
		syscall1(SyscallID::DebugConsolePutchar, c as usize);
	}
}
