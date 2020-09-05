#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(jr_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use jr_os::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    jr_os::init();
    println!("Hello World{}", "!");

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // uncomment line below to trigger a stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jr_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}