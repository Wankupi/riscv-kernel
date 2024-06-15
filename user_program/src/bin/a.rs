#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

extern crate sys;

#[no_mangle]
extern "C" fn main(a0: usize) -> isize {
	let c = (a0 + 'a' as usize) as u8;
	let uart = 0x10000000 as *mut u8;
	loop {
		unsafe {
			uart.write_volatile(c);
		}
	}
	return 0;
}
