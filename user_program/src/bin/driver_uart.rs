#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

const BUFFER_SIZE: usize = 1024;

static mut buffer_committed: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut buffer_type: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut buffer_write: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

const UartAddr: usize = 0x1000_0000;

fn get_lsr() -> u8 {
	unsafe { ((UartAddr + 0x5) as *const u8).read_volatile() }
}
fn readable() -> bool {
	get_lsr() & 0x1 != 0
}

fn putchar(c: u8) {
	unsafe { (UartAddr as *mut u8).write_volatile(c) };
}

pub use sys::*;

#[no_mangle]
extern "C" fn main() -> isize {


	return 0;
}
