use core::fmt::{self, Write};

use crate::driver::uart::uart_device;

#[macro_export]
macro_rules! function_name {
	() => {{
		fn f() {}
		fn type_name_of<T>(_: T) -> &'static str {
			core::any::type_name::<T>()
		}
		let name = type_name_of(f);

		// Find and cut the rest of the path
		match &name[..name.len() - 3].rfind(':') {
			Some(pos) => &name[pos + 1..name.len() - 3],
			None => &name[..name.len() - 3],
		}
	}};
}

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
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::print::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::print::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! debugmsg {
	($level: literal, $unit: expr, $fmt: literal $(, $($arg: tt)+)?) => {
		println!(concat!("\x1b[{}m", "[{:^16}]", "\x1b[0m" ," ", $fmt), $level, $unit $(, $($arg)+)?);
	};
}

#[macro_export]
macro_rules! success {
	($fmt: expr $(, $($arg: tt)+)?) => {
		debugmsg!(32, function_name!(), $fmt $(, $($arg)+)?);
	};
}

#[macro_export]
macro_rules! info {
	($fmt: expr $(, $($arg: tt)+)?) => {
		debugmsg!(34, function_name!(), $fmt $(, $($arg)+)?);
	};
}

#[macro_export]
macro_rules! error {
	($fmt: expr $(, $($arg: tt)+)?) => {
		debugmsg!(31, function_name!(), $fmt $(, $($arg)+)?);
	};
}

#[macro_export]
macro_rules! log {
	($fmt: expr $(, $($arg: tt)+)?) => {
		debugmsg!(33, function_name!(), $fmt $(, $($arg)+)?);
	};
}
