use core::mem::size_of;

use lazy_static::lazy_static;

use crate::memory::address::VirtualAddress;

const STACK_SIZE: usize = 1 << 12; // 4KB

pub const PRIVILEGE_LEVEL_ZERO_STACK_TABLE_INDEX: usize = 0x00;
pub const PRIVILEGE_LEVEL_THREE_STACK_TABLE_INDEX: usize = 0x02;

pub const DEBUG_STACK_TABLE_INDEX: usize = 0x00;
pub const NMI_STACK_TABLE_INDEX: usize = 0x01;
pub const DOUBLE_FAULT_STACK_TABLE_INDEX: usize = 0x02;
pub const STACK_SEGMENT_FAULT_STACK_TABLE_INDEX: usize = 0x03;
pub const GENERAL_PROTECTION_STACK_TABLE_INDEX: usize = 0x04;
pub const MACHINE_CHECK_STACK_TABLE_INDEX: usize = 0x05;
// I'll revisit this once I get to memory management
// const PAGE_FAULT_STACK_TABLE_INDEX: usize = 0x07;

enum StackTableType {
    Privilege,
    Interrupt,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
pub struct TaskStateSegment {
    reserved_1: u32,
    privilege_stack_table: [VirtualAddress; 3],
    reserved_2: u64,
    interrupt_stack_table: [VirtualAddress; 7],
    reserved_3: u64,
    reserved_4: u16,
    iomap_base: u16,
}

impl TaskStateSegment {
    pub fn new() -> TaskStateSegment {
        TaskStateSegment {
            reserved_1: 0,
            privilege_stack_table: [VirtualAddress::zero(); 3],
            reserved_2: 0,
            interrupt_stack_table: [VirtualAddress::zero(); 7],
            reserved_3: 0,
            reserved_4: 0,
            iomap_base: (size_of::<TaskStateSegment>() - 1) as u16,
        }
    }

    pub fn pointer(&self) -> VirtualAddress {
        VirtualAddress::new(self as *const _ as u64)
    }

    fn init_stack_table(&mut self, stack_table_index: usize, stack_table_type: StackTableType) {
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_ptr: u64 = unsafe { &STACK as *const _ as u64 } + STACK_SIZE as u64;
        let canonical_stack_ptr = VirtualAddress::new(stack_ptr);
        match stack_table_type {
            StackTableType::Privilege => {
                debug_assert!(stack_table_index < 3);
                self.privilege_stack_table[stack_table_index] = canonical_stack_ptr
            }
            StackTableType::Interrupt => {
                debug_assert!(stack_table_index < 7);
                self.interrupt_stack_table[stack_table_index] = canonical_stack_ptr
            }
        }
    }
}

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // Intel Software Developer's Manual - section 6.14.5
        // See https://www.kernel.org/doc/Documentation/x86/kernel-stacks
        // Privilege Stacks
        tss.init_stack_table(PRIVILEGE_LEVEL_ZERO_STACK_TABLE_INDEX, StackTableType::Privilege);
        tss.init_stack_table(PRIVILEGE_LEVEL_THREE_STACK_TABLE_INDEX, StackTableType::Privilege);

        // Interrupt Stacks
        tss.init_stack_table(DEBUG_STACK_TABLE_INDEX, StackTableType::Interrupt);
        tss.init_stack_table(NMI_STACK_TABLE_INDEX, StackTableType::Interrupt);
        tss.init_stack_table(DOUBLE_FAULT_STACK_TABLE_INDEX, StackTableType::Interrupt);
        tss.init_stack_table(STACK_SEGMENT_FAULT_STACK_TABLE_INDEX, StackTableType::Interrupt);
        tss.init_stack_table(GENERAL_PROTECTION_STACK_TABLE_INDEX, StackTableType::Interrupt);
        tss.init_stack_table(MACHINE_CHECK_STACK_TABLE_INDEX, StackTableType::Interrupt);
        tss
    };
}
