#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::arch::asm;
use core::panic::PanicInfo;

pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod structs;
pub mod vga;

pub fn kernel_init() {
    println!("Loading GDT...");
    structs::gdt::init_gdt();
    println!("...[ok]");
    println!("Loading IDT...");
    structs::idt::init_idt();
    println!("...[ok]");
}

// TODO: This needs some TLC. Write a good port struct with methods that make sense
#[derive(Debug)]
#[repr(transparent)]
pub struct Port(u16);

impl Port {
    unsafe fn write(&self, value: u32) {
        let port = self.0;
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

// Exit utils
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit(exit_code: ExitCode) {
    unsafe {
        let port = Port(0xf4);
        port.write(exit_code as u32);
    }
}

// Test Framework
pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit(ExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Panicked at: {}", info);
    exit(ExitCode::Failure);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
