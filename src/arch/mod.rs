use core::arch::asm;
pub mod regs;
pub mod trap;

pub fn shutdown() -> ! {
	// info!("shutdown");
	unsafe {
		*(0x100000 as *mut u32) = 0x5555;
	}
	loop {}
}

pub fn get_hart_id() -> usize {
	let hart_id: usize;
	unsafe { asm!("mv {}, tp", out(reg) hart_id) }
	hart_id
}

pub fn get_kaslr_seed(_dtb_pa: usize) -> usize {
	0
}

pub fn get_clock() -> u64 {
	let time: u64;
	// unsafe { asm!("csrr {}, time", out(reg) time) }
	unsafe { asm!("rdtime {}", out(reg) time) }
	time
}

pub fn get_tp() -> usize {
	let tp: usize;
	unsafe { asm!("mv {}, tp", out(reg) tp) }
	tp
}

pub fn set_timer() {
	const MS: u64 = 30000;
	sbi_rt::set_timer(get_clock() + MS);
}
