#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

use core::{fmt::{self, Write}, panic::PanicInfo, usize};
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
	pub fn msg_send(key: usize, data: &[u8]) -> isize {
		let r = syscall3(SyscallID::MsgSend, key, data.as_ptr() as usize, data.len());
		r as isize
	}
	pub fn msg_recv(key: usize, buf: &mut [u8]) -> isize {
		let len = buf.len();
		let r = syscall3(SyscallID::MsgRecv, key, buf.as_ptr() as usize, len);
		r as isize
	}
	pub fn fork() -> isize {
		let r = syscall1(SyscallID::Fork, 0);
		r as isize
	}
	pub fn exec(name: &[u8]) -> isize {
		let r = syscall3(SyscallID::Exec, name.as_ptr() as usize, name.len(), 0);
		r as isize
	}
	pub fn fork_exec(name: &[u8]) -> isize {
		let r = syscall3(SyscallID::ForkExec, name.as_ptr() as usize, name.len(), 0);
		r as isize
	}
	pub fn wait_pid(pid: isize) -> isize {
		let r = syscall1(SyscallID::Wait, pid as usize);
		r as isize
	}
}

pub use syscall::*;

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;
pub const STDERR: usize = 2;



pub fn printk(s: &[u8]) {
	syscall::write(STDOUT, s);
}

struct Stdout;

impl Write for Stdout {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		printk(s.as_bytes());
		Ok(())
	}
}

pub fn print(args: fmt::Arguments) {
	Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
