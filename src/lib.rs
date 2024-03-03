#![no_std]
#![no_main]

mod arch;
mod lang_items;

#[no_mangle]
pub static dtb_addr: usize = 0x12345678;

fn put_hex(mut n : usize) {
	let mut s = [0u8; 16];
	let mut i = 0;
	while i < 16 {
		let d = n & 0xf;
		s[15 - i] = if d < 10 { b'0' + d as u8 } else { b'a' + (d - 10) as u8 };
		i += 1;
		n >>= 4;
	}
	arch::printk(&s);
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	let s: &'static str = "Hello, world!\n";
	arch::printk(s.as_bytes());
	put_hex(dtb_addr);
	arch::shutdown();
}
