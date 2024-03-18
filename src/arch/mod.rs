use crate::info;

pub mod trap;
pub mod mm;

pub fn shutdown() -> ! {
	info!("shutdown");
	unsafe { *(0x100000 as *mut u32) = 0x5555; }
	loop {}
}