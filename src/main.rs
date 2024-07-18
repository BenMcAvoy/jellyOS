#![no_std] // No standard library
#![no_main] // No default entrypoint

#![feature(custom_test_frameworks)] // Custom test framework support
#![test_runner(jellyos::test_runner)] // Test runner
#![reexport_test_harness_main = "test_main"] // Test main function

use core::panic::PanicInfo;
use jellyos::{print, println};

const BANNER_TOP: &str = r#" ________________
< jellyOS v"#;

const BANNER: &str = r#" >
 ----------------
 \
  \
   \ >()_
      (__)__ _"#;

#[cfg(test)]
use jellyos::{serial_println, qemu};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    jellyos::init();

    #[cfg(test)]
    test_main();

    println!("{BANNER_TOP}{}{BANNER}\n", env!("CARGO_PKG_VERSION"));

    jellyos::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(not(test))]
use jellyos::vga_buffer::{Colour, ColourCode, WRITER};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    {
        // Separate scope to avoid deadlocks where the WRITER lock is held and println! tries to acquire it
        let mut writer = WRITER.lock();
        writer.colour_code = ColourCode::new(Colour::White, Colour::Red);
        writer.column_position = 0;
    }

    println!("\n\n{}", info);

    jellyos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    jellyos::hlt_loop();
}
