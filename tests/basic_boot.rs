#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jellyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use jellyos::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jellyos::test_panic_handler(info)
}
