use sbi_rt::{console_write, system_reset, NoReason, Physical, Shutdown};

pub fn printk(s: &[u8]) {
	let ptr = s.as_ptr();
	let bytes = Physical::<&[u8]>::new(s.len(), ptr as usize, ptr as usize >> 32);
	console_write(bytes);
}

pub fn shutdown() -> ! {
	system_reset(Shutdown, NoReason);
	loop {}
}
