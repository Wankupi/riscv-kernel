pub mod process;
pub mod scheduler;
pub mod syscall;
pub mod task;
pub mod trapframe;

pub mod elf_funcs_gen;
use elf_funcs_gen::*;
use task::Task;

use crate::{
	_copy_u_s, _trap_entry, copy_u_s,
	mm::vm::{vm_map, PTE},
};

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

fn copy_from_user(kdst: &mut [u8], usrc: *const u8, len: usize, task: &mut Task) -> isize {
	if kdst.len() < len {
		panic!("copy_from_user: buffer is too small");
	}
	let vt = task.process.pagetable.as_mut();
	let pa = kdst.as_mut_ptr() as usize;
	let offset = pa & 0xfff;
	vm_map(vt, 0xffffffff_0000_0000, pa ^ offset, len + offset, PTE::RW);
	copy_u_s(
		usrc as *mut u8,
		0xffffffff_0000_0000 as *mut u8,
		len,
		task.process.trapframe.satp,
	)
}

fn copy_to_user(udst: *mut u8, ksrc: &[u8], len: usize, task: &mut Task) -> isize {
	if ksrc.len() < len {
		panic!("copy_to_user: buffer is too small");
	}
	let vt = task.process.pagetable.as_mut();
	let pa = ksrc.as_ptr() as usize;
	let offset = pa & 0xfff;
	vm_map(vt, 0xffffffff_0000_0000, pa ^ offset, len + offset, PTE::RW);
	copy_u_s(
		0xffffffff_0000_0000 as *mut u8,
		udst as *mut u8,
		len,
		task.process.trapframe.satp,
	)
}
