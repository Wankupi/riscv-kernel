use fdt::Fdt;

use crate::driver::uart::uart_device;

static mut global_fdt: Option<Fdt<'static>> = None;

pub fn init_fdt(dtb_addr: *const u8) {
	unsafe { global_fdt = Some(Fdt::from_ptr(dtb_addr).unwrap()) };
}

pub fn init_stdout() {
	let fdt = unsafe { global_fdt.as_ref().expect("fdt did not initialize") };
	let stdout = fdt.find_node("serial0").expect("serial 0 should exist");
	let mut reg = stdout.reg().expect("stdout does not have reg property");
	let mem_region = reg.next().expect("stdout size property is empty");
	let io_width = stdout.property("reg-io-width").map_or(1, |p| {
		let bytes: [u8; 4] = p.value.try_into().expect("reg-io-width must be 4 bytes");
		u32::from_be_bytes(bytes) as usize
	});
	let reg_shift = stdout.property("reg-shift").map_or(0, |p| {
		let bytes: [u8; 4] = p.value.try_into().expect("reg-shift must be 4 bytes");
		u32::from_be_bytes(bytes) as usize
	});
	unsafe {
		uart_device.init_with_config(mem_region.starting_address, io_width, reg_shift);
	}
}
