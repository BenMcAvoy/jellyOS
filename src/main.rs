#![no_std] // No standard library
#![no_main] // No default entrypoint

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // Never stop
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {} // Never stop
}
