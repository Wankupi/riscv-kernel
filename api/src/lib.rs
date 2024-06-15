#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

#[derive(Debug)]
pub enum SyscallID {
	Fork = 57,
	DebugConsoleWrite = 512,
	DebugConsolePutchar = 513,
}


impl From<usize> for SyscallID {
	fn from(id: usize) -> Self {
		match id {
			57 => SyscallID::Fork,
			512 => SyscallID::DebugConsoleWrite,
			513 => SyscallID::DebugConsolePutchar,
			_ => panic!("unknown syscall id: {}", id),
		}
	}
}
