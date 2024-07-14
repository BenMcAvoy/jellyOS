use crate::println;
use crate::vga_buffer::*;

use crate::qemu;
use crate::serial_println;

use core::panic::PanicInfo;

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

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    loop {}
}
