#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![test_runner(flap_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use flap_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    flap_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    flap_os::kernel_init();

    #[cfg(test)]
    test_main();

    loop {}
}
