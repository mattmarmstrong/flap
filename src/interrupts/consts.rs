// Intel Manual - Section 6.3.1
// These are the vector numbers for the pre-defined interrupts
pub const DIVIDE_ERROR: usize = 0x00;
pub const DEBUG_EXCEPTION: usize = 0x01;
pub const NMI_INTERRUPT: usize = 0x02; // Non-maskable (hardware) Interrupt
pub const BREAKPOINT: usize = 0x03;
pub const OVERFLOW: usize = 0x04;
pub const BOUND_RANGE_EXCEEDED: usize = 0x05;
pub const INVALID_OPCODE: usize = 0x06;
pub const DEVICE_NOT_AVAILABLE: usize = 0x07;
pub const DOUBLE_FAULT: usize = 0x08;
pub const COPROCESSOR_SEGMENT_OVERRUN: usize = 0x09; //reserved, not used
pub const INVALID_TSS: usize = 0x0A;
pub const SEGMENT_NOT_PRESENT: usize = 0x0B;
pub const STACK_SEGMENT_FAULT: usize = 0x0C;
pub const GENERAL_PROTECTION: usize = 0x0D;
pub const PAGE_FAULT: usize = 0x0E;
// Vector #15 is reserved and not in use by modern x86_64 processors
pub const X87_FLOATING_POINT_ERROR: usize = 0x10;
pub const ALIGNMENT_CHECK: usize = 0x11;
pub const MACHINE_CHECK: usize = 0x12;
pub const SIMD_FLOATING_POINT_EXCEPTION: usize = 0x13;
pub const VIRTUALIZATION_EXCEPTION: usize = 0x14;
// Vectors #21-31 are reserved, and #32-255 are reserved for user defined interrupts
