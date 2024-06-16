#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]

macro_rules! enum_from {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident = $val:expr,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname = $val,)*
        }

        impl From<usize> for $name {
            fn from(id: usize) -> Self {
                match id {
                    $($val => $name::$vname,)*
                    _ => panic!("unknown syscall id: {}", id),
                }
            }
        }
    }
}

enum_from! {
#[derive(Debug)]
pub enum SyscallID {
	Fork = 57,
    Read = 63,
    Write = 64,
	Exit = 93,
    MsgGet = 186,
	MsgSend = 187,
	MsgRecv = 188,
	DebugConsoleWrite = 512,
	DebugConsolePutchar = 513,
}
}
