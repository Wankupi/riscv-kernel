#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

use sys::*;

fn getchar() -> u8 {
	let mut buf = [0 as u8; 1];
	read(STDIN, &mut buf);
	return buf[0];
}

#[no_mangle]
extern "C" fn main() -> isize {
	write(STDOUT, b"Shell Started\n");
	loop {
		write(STDOUT, b"\x1b[32m>\x1b[0m ");
		let mut buf = [0u8; 1024];
		let mut len = 0;
		let mut c = getchar();
		while c != b'\n' {
			buf[len] = c;
			len += 1;
			c = getchar();
		}
		if len == 0 {
			continue;
		}
		if buf[..len] == *b"exit" {
			break;
		}
		// let r = fork();
		// if r == 0 {
		// 	exec(&buf[..len]);
		// }
		let r = fork_exec(&buf[..len]);
		if r <= 0 {
			write(STDOUT, b"Command not found\n");
		} else {
			let code = wait_pid(r);
			print!("\x1b[33m%{}\x1b[0m\n", code);
		}
	}
	write(STDOUT, b"Shell Exited\n");
	return 0;
}
