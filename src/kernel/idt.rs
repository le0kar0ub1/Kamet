use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::kernel::exceptions;
use crate::kernel::pic;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[0].set_handler_fn(exceptions::division_by_zero_handler);
        idt.debug.set_handler_fn(exceptions::debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(exceptions::non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
        idt.overflow.set_handler_fn(exceptions::overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(exceptions::bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(exceptions::invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(exceptions::device_not_available_handler);
        idt.double_fault.set_handler_fn(exceptions::double_fault_handler);
        idt.invalid_tss.set_handler_fn(exceptions::invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(exceptions::segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(exceptions::stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(exceptions::general_protection_fault_handler);
        idt.page_fault.set_handler_fn(exceptions::page_fault_handler);
        idt.x87_floating_point.set_handler_fn(exceptions::x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(exceptions::alignment_check_handler);
        idt.simd_floating_point.set_handler_fn(exceptions::simd_floating_point_handler);
        idt.virtualization.set_handler_fn(exceptions::virtualization_handler);
        idt.security_exception.set_handler_fn(exceptions::security_exception_handler);
        idt[32].set_handler_fn(pic::timer_handler);
        // idt[33].set_handler_fn(pic::keyboard_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

// type X86handler = extern "x86-interrupt" fn(stack_frame: &mut InterruptStackFrame);