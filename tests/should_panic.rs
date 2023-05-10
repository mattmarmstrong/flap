#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use flap_os::{exit, serial_println, ExitCode};
use x86_64::instructions::hlt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {
        hlt();
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit(ExitCode::Failure);
    }
    exit(ExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit(ExitCode::Success);
    loop {
        hlt();
    }
}
