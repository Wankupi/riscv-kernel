#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

use sys::*;

#[no_mangle]
extern "C" fn main() -> isize {
	let mut buff = [0u8; 1024];
	let mut len = 0;
	len = sys::read(STDIN, buff.as_mut());
	println!("read {} bytes {}", len, buff[0]);
	return 0;
}
