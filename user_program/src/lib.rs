#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

use core::{panic::PanicInfo, usize};
extern crate SyscallAPI;

extern "C" {
	fn main(a0: usize, a1: usize, a2: usize) -> isize;
}

#[no_mangle]
extern "C" fn _start(a0: usize, a1: usize, a2: usize) {
	let code = unsafe { main(a0, a1, a2) };
	syscall::exit(code);
	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

pub mod syscall {
	use core::arch::asm;
	use SyscallAPI::SyscallID;
	// calling convention for syscalls
	// a7: syscall number
	// a0-a6: arguments
	fn syscall1(syscall_id: SyscallID, args1: usize) -> usize {
		let ret: usize;
		unsafe {
			asm!("ecall", in("a7") syscall_id as usize, in("a0") args1, lateout("a0") ret);
		}
		ret
	}
	fn syscall3(syscall_id: SyscallID, args1: usize, args2: usize, args3: usize) -> usize {
		let ret: usize;
		unsafe {
			asm!("ecall", in("a7") syscall_id as usize, in("a0") args1, in("a1") args2, in("a2") args3, lateout("a0") ret);
		}
		ret
	}

	pub fn debug_console_write(s: &str) {
		syscall1(SyscallID::DebugConsoleWrite, s.as_ptr() as usize);
	}
	pub fn debug_console_putchar(c: u8) {
		syscall1(SyscallID::DebugConsolePutchar, c as usize);
	}
	pub fn exit(code: isize) {
		syscall1(SyscallID::Exit, code as usize);
	}
	pub fn write(fd: usize, buf: &[u8]) -> isize {
		let r = syscall3(SyscallID::Write, fd, buf.as_ptr() as usize, buf.len());
		r as isize
	}
	pub fn read(fd: usize, buf: &mut [u8]) -> isize {
		let len = buf.len();
		let r = syscall3(SyscallID::Read, fd, buf.as_ptr() as usize, len);
		r as isize
	}
}

pub use syscall::*;

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;
pub const STDERR: usize = 2;
