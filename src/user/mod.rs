pub mod process;
pub mod scheduler;
pub mod task;
pub mod trapframe;

pub mod elf_funcs_gen;
use elf_funcs_gen::*;

pub fn get_userapp_by_name(name: &str) -> Option<&'static [u8]> {
	for i in 0..userapps_count {
		if userapps_name[i] == name {
			return Some(unsafe {
				core::slice::from_raw_parts(usreapps_addr[i], userapps_size[i])
			});
		}
	}
	return None;
}
