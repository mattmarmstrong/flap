use crate::println;

#[derive(Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    // The pointers are technically VirtualAddresses, but we know that, so who cares
    instruction_pointer: u64,
    segment_selector: u64, // needs to be a padded SegmentSelector
    rflags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: DOUBLE FAULT");
    println!("ERROR CODE: {:#?}", error_code);
    println!("{:#?}", stack_frame);
    panic!();
}

pub extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: SEGMENT NOT PRESENT");
    println!("ERROR CODE: {:#?}", error_code);
    println!("{:#?}", stack_frame);
    panic!();
}

pub extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: STACK SEGMENT FAULT");
    println!("ERROR CODE: {:#?}", error_code);
    println!("{:#?}", stack_frame);
    panic!();
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT");
    println!("ERROR CODE: {:#?}", error_code);
    println!("{:#?}", stack_frame);
    panic!();
}
