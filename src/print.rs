// use crate::arch::sbi::printk;
use core::fmt::{self, Write};

use crate::driver::uart::uart_device;

pub fn printk(s: &[u8]) {
	let mut ptr = s.as_ptr();
	let end = s.as_ptr().wrapping_add(s.len());
	while ptr != end {
		unsafe { uart_device.write(*ptr) }
		ptr = ptr.wrapping_add(1);
	}
}

struct Stdout;

impl Write for Stdout {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		printk(s.as_bytes());
		Ok(())
	}
}

pub fn print(args: fmt::Arguments) {
	Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
