#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate rlibc;
use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}