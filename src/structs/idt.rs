use core::arch::asm;
use core::mem::size_of;

use lazy_static::lazy_static;

use crate::interrupts::consts::*;
use crate::interrupts::interrupt_handlers::*;
use crate::memory::address::VirtualAddress;
use crate::structs::gdt::{SegmentSelector, GDT};
use crate::structs::tss::*;

#[derive(Debug, Clone)]
#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
    pub descriptor_table: [GateDescriptor; 256],
}

impl InterruptDescriptorTable {
    fn new() -> Self {
        // These are all of the gates I have defined (very basic) handlers for so far.
        InterruptDescriptorTable {
            descriptor_table: [GateDescriptor::new(GateOptions::minimal()); 256],
        }
    }

    fn pointer(&self) -> IdtPointer {
        let limit = (self.descriptor_table.len() * size_of::<GateDescriptor>() - 1) as u16;
        let base = VirtualAddress::new(self.descriptor_table.as_ptr() as u64);
        IdtPointer { limit, base }
    }

    pub unsafe fn load_idt(idt_ptr: &IdtPointer) {
        asm!("lidt [{}]", in(reg) idt_ptr, options(readonly, nostack, preserves_flags))
    }
}

#[repr(C, packed(2))]
pub struct IdtPointer {
    limit: u16,
    base: VirtualAddress,
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
    fn new(gate_options: GateOptions) -> Self {
        GateDescriptor {
            offset_low: 0,
            segment: GDT.1.kernel_code_segment,
            gate_options,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    fn set_handler_address(mut self, handler_address: VirtualAddress) -> Self {
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

    fn set_present(mut self) -> Self {
        self.0 |= 0x8000;
        self
    }

    fn dpl_0(mut self) -> Self {
        self.0 &= 0x9FFF;
        self
    }

    fn dpl_3(mut self) -> Self {
        self.0 &= 0xCFFF;
        self
    }

    fn enable_interrupts(mut self) -> Self {
        self.0 |= 0x0100;
        self
    }

    fn default() -> Self {
        GateOptions::minimal()
            .set_present()
            .dpl_0()
            .enable_interrupts()
    }

    fn set_stack_index(mut self, stack_index: usize) -> Self {
        debug_assert!(stack_index < 6);
        self.0 = (self.0 & 0xFFF8) | ((stack_index + 1) as u16);
        self
    }
}

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        let debug_exception_gate_descriptor =
            GateDescriptor::new(GateOptions::default().set_stack_index(DEBUG_STACK_TABLE_INDEX))
                .set_handler_address(VirtualAddress::new(
                    (debug_exception_handler as usize) as u64,
                ));
        let nmi_gate_descriptor =
            GateDescriptor::new(GateOptions::default().set_stack_index(NMI_STACK_TABLE_INDEX))
                .set_handler_address(VirtualAddress::new((nmi_handler as usize) as u64));
        let breakpoint_gate_descriptor = GateDescriptor::new(GateOptions::default())
            .set_handler_address(VirtualAddress::new(
                (breakpoint_exception_handler as usize) as u64,
            ));
        let segment_not_present_descriptor = GateDescriptor::new(GateOptions::default())
            .set_handler_address(VirtualAddress::new(
                (segment_not_present_handler as usize) as u64,
            ));
        let double_fault_descriptor = GateDescriptor::new(
            GateOptions::default().set_stack_index(DOUBLE_FAULT_STACK_TABLE_INDEX),
        )
        .set_handler_address(VirtualAddress::new((double_fault_handler as usize) as u64));

        let stack_segment_fault_descriptor = GateDescriptor::new(
            GateOptions::default().set_stack_index(STACK_SEGMENT_FAULT_STACK_TABLE_INDEX),
        )
        .set_handler_address(VirtualAddress::new(
            (stack_segment_fault_handler as usize) as u64,
        ));
        let general_protection_fault_gate_descriptor = GateDescriptor::new(
            GateOptions::default().set_stack_index(GENERAL_PROTECTION_STACK_TABLE_INDEX),
        )
        .set_handler_address(VirtualAddress::new(
            (general_protection_fault_handler as usize) as u64,
        ));
        idt.descriptor_table[DEBUG_EXCEPTION] = debug_exception_gate_descriptor;
        idt.descriptor_table[NMI_INTERRUPT] = nmi_gate_descriptor;
        idt.descriptor_table[BREAKPOINT] = breakpoint_gate_descriptor;
        idt.descriptor_table[SEGMENT_NOT_PRESENT] = segment_not_present_descriptor;
        idt.descriptor_table[DOUBLE_FAULT] = double_fault_descriptor;
        idt.descriptor_table[STACK_SEGMENT_FAULT] = stack_segment_fault_descriptor;
        idt.descriptor_table[GENERAL_PROTECTION] = general_protection_fault_gate_descriptor;
        idt
    };
}

pub fn init_idt() {
    unsafe {
        InterruptDescriptorTable::load_idt(&IDT.pointer());
    }
}
