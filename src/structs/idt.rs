use lazy_static::lazy_static;

use crate::{addr::VirtualAddress, structs::gdt::SegmentSelector};

#[derive(Debug, Clone)]
#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
    // These are all the reserved, architecture defined interrupts
    pub divide_by_zero_descriptor: GateDescriptor,
    pub debug_exception_descriptor: GateDescriptor,
    pub nmi_interrupt_descriptor: GateDescriptor,
    pub breakpoint_descriptor: GateDescriptor,
    pub overflow_descriptor: GateDescriptor,
    pub bound_range_exceeded_descriptor: GateDescriptor,
    pub invalid_opcode_descriptor: GateDescriptor,
    pub device_not_available_descriptor: GateDescriptor,
    pub double_fault_descriptor: GateDescriptor,
    pub coprocessor_segment_overrun_descriptor: GateDescriptor,
    pub invalid_tss_descriptor: GateDescriptor,
    pub segment_not_present_descriptor: GateDescriptor,
    pub stack_segment_fault_descriptor: GateDescriptor,
    pub general_protection_descriptor: GateDescriptor,
    pub page_fault_descriptor: GateDescriptor,
    reserved_int_15: GateDescriptor, // this should never be used
    pub x87_floating_point_error_descriptor: GateDescriptor,
    pub alignment_check_descriptor: GateDescriptor,
    pub machine_check_descriptor: GateDescriptor,
    pub simd_floating_point_exception_descriptor: GateDescriptor,
    pub virtualization_exception: GateDescriptor,
    reserved_int_21_to_31: [GateDescriptor; 10],
    // As the name suggests, these are free to define. TODO: device I/O
    pub user_defined_interrupts: [GateDescriptor; 256 - 32],
}

impl InterruptDescriptorTable {
    fn new() -> Self {
        InterruptDescriptorTable {
            // The types of some of these are wrong -> https://wiki.osdev.org/Interrupt_Descriptor_Table
            divide_by_zero_descriptor: GateDescriptor::new(),
            debug_exception_descriptor: GateDescriptor::new(),
            nmi_interrupt_descriptor: GateDescriptor::new(),
            breakpoint_descriptor: GateDescriptor::new(),
            overflow_descriptor: GateDescriptor::new(),
            bound_range_exceeded_descriptor: GateDescriptor::new(),
            invalid_opcode_descriptor: GateDescriptor::new(),
            device_not_available_descriptor: GateDescriptor::new(),
            double_fault_descriptor: GateDescriptor::new(),
            coprocessor_segment_overrun_descriptor: GateDescriptor::new(),
            invalid_tss_descriptor: GateDescriptor::new(),
            segment_not_present_descriptor: GateDescriptor::new(),
            stack_segment_fault_descriptor: GateDescriptor::new(),
            general_protection_descriptor: GateDescriptor::new(),
            page_fault_descriptor: GateDescriptor::new(),
            reserved_int_15: GateDescriptor::new(), // this should never be used
            x87_floating_point_error_descriptor: GateDescriptor::new(),
            alignment_check_descriptor: GateDescriptor::new(),
            machine_check_descriptor: GateDescriptor::new(),
            simd_floating_point_exception_descriptor: GateDescriptor::new(),
            virtualization_exception: GateDescriptor::new(),
            reserved_int_21_to_31: [GateDescriptor::new(); 10],

            user_defined_interrupts: [GateDescriptor::new(); 224],
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct GateDescriptor {
    offset_low: u16,
    segment: SegmentSelector,
    gate_options: GateOptions,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl GateDescriptor {
    fn new() -> Self {
        GateDescriptor {
            offset_low: 0,
            segment: SegmentSelector::new(0, 0), // This points at the null segment
            gate_options: GateOptions::minimal(),
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    fn set_handler_address(&mut self, handler_address: VirtualAddress) -> &mut Self {
        self.offset_low = (handler_address.0 & 0x0000_0000_0000_FFFF) as u16;
        self.offset_middle = ((handler_address.0 & 0x0000_0000_FFFF_0000) >> 16) as u16;
        self.offset_high = ((handler_address.0 & 0xFFFF_FFFF_0000_0000) >> 32) as u32;
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct GateOptions(u16);

impl GateOptions {
    fn minimal() -> Self {
        GateOptions(0x0E00)
    }

    fn new(present: bool, interruptable: bool) -> Self {
        let mut options = GateOptions::minimal();
        options
            .set_present(present)
            .disable_interrupts(interruptable);
        options
    }

    fn set_present(&mut self, present: bool) -> &mut Self {
        match present {
            false => self.0 = (self.0 << 1) >> 1,
            true => self.0 |= 0x8000,
        }
        self
    }

    fn set_stack_index(&mut self, stack_index: u16) -> &mut Self {
        if stack_index > 6 {
            panic!("Invalid stack pointer passed to interrupt handler")
        } else {
            self.0 = (self.0 & 0xFFF8) | (stack_index + 1);
        }
        self
    }

    fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        let privilege_mask: u16 = 0x0003;
        self.0 = (self.0 & 0x9FFF) | ((dpl & privilege_mask) << 13); // 0 <= privilege level <= 3
        self
    }

    fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        match disable {
            false => self.0 |= 0x0100,
            true => self.0 &= !0x0100,
        }
        self
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt
    };
}
