
// Intel Manual - Section 6.3.1
// These are the vector numbers for the pre-defined interrupts
pub const DIVIDE_ERROR: u8 = 0x00;
pub const DEBUG_EXCEPTION: u8 = 0x01;
pub const NMI_INTERRUPT: u8 = 0x02; // Non-maskable (hardware) Interrupt
pub const BREAKPOINT: u8 = 0x03;
pub const OVERFLOW: u8 = 0x04;
pub const BOUND_RANGE_EXCEEDED: u8 = 0x05;
pub const INVALID_OPCODE: u8 = 0x06;
pub const DEVICE_NOT_AVAILABLE: u8 = 0x07;
pub const DOUBLE_FAULT: u8 = 0x08;
pub const COPROCESSOR_SEGMENT_OVERRUN: u8 = 0x09; //reserved, not used
pub const INVALID_TSS: u8 = 0x0A;
pub const SEGMENT_NOT_PRESENT: u8 = 0x0B;
pub const STACK_SEGMENT_FAULT: u8 = 0x0C;
pub const GENERAL_PROTECTION: u8 = 0x0D;
pub const PAGE_FAULT: u8 = 0x0E;
// Vector #15 is reserved and not in use by modern x86_64 processors
pub const X87_FLOATING_POINT_ERROR: u8 = 0x10;
pub const ALIGNMENT_CHECK: u8 = 0x11;
pub const MACHINE_CHECK: u8 = 0x12;
pub const SIMD_FLOATING_POINT_EXCEPTION: u8 = 0x13;
pub const VIRTUALIZATION_EXCEPTION: u8 = 0x14;
// Vectors #21-31 are reserved, and #32-255 are reserved for user defined interrupts


