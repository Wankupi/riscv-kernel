#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

use sys::{msg_recv, write, STDOUT};

#[no_mangle]
extern "C" fn main() -> isize {
	let mut buff = [0 as u8; 64];
	loop {
		msg_recv(1, buff.as_mut_slice());
		write(
			STDOUT,
			core::str::from_utf8(buff.as_slice()).unwrap().as_bytes(),
		);
	}
	return 0;
}
