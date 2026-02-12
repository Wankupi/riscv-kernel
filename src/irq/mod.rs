use crate::{arch::get_hart_id, PLIC};

pub const UART_IRQ: u32 = 10;
pub const VIRT_IO: u32 = 1;

fn plic_enable_interrupt(contextId: u32, id: u32) {
	unsafe {
		let addr = PLIC + 0x2000 + contextId as usize * 0x80 + (id / 32) as usize * 4;
		let ptr = addr as *mut u32;
		let old_val = ptr.read_volatile();
		let new_val = old_val | 1u32 << (id % 32);
		ptr.write_volatile(new_val);
	}
}

fn plic_set_priority(id: u32, pri: u32) {
	unsafe {
		((PLIC + id as usize * 4) as *mut u32).write_volatile(pri);
	}
}

fn plic_set_context_threshold(contextId: u32, threshold: u32) {
	unsafe {
		((PLIC + 0x20_0000 + contextId as usize * 0x1000) as *mut u32).write_volatile(threshold);
	}
}

pub fn plic_get_context_claim(contextId: u32) -> u32 {
	unsafe { ((PLIC + 0x20_0004 + contextId as usize * 0x1000) as *mut u32).read_volatile() }
}
pub fn plic_complete(contextId: u32, id: u32) {
	unsafe {
		((PLIC + 0x20_0004 + contextId as usize * 0x1000) as *mut u32).write_volatile(id);
	}
}

pub fn plic_init() {
	plic_set_priority(UART_IRQ, 1); // Set the priority of the UART IRQ to 1
	plic_set_priority(VIRT_IO, 1); // Set the priority of the UART IRQ to 1

	plic_set_context_threshold(0, 1); // Set the threshold of context 0 to 0
	plic_set_context_threshold(1, 0); // Set the threshold of context 0 to 0

	plic_enable_interrupt(1, UART_IRQ);
	plic_enable_interrupt(1, VIRT_IO);
}
