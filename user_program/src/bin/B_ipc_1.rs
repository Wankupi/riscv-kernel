#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

extern crate sys;
use sys::*;

#[no_mangle]
extern "C" fn main() -> isize {
	msg_send(1, b"Hello, this is an ipc message.");
	return 0;
}
