use core::panic::PanicInfo;
use crate::error;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    error!("Panic: {}\n", _info);
    loop {}
}