#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![test_runner(flap_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use flap_os::memory::paging::page_table::read_cr3;
use flap_os::println;
entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    flap_os::kernel_init();

    println!("CR3 value after boot: {:#?}", read_cr3());
    #[cfg(test)]
    test_main();
    loop {}
}

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
