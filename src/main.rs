#![no_std] // No standard library
#![no_main] // No default entrypoint

mod panic;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "Hello, world! The numbers are {} and {}\nHello again!",
        42,
        1.0 / 3.0
    );

    None::<Option<i32>>.unwrap();

    loop {}
}
