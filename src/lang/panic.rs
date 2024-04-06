use crate::error;
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	error!("Panic: {}\n", _info);
	loop {}
}
