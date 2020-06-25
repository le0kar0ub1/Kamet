use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::kernel::exceptions;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[0].set_handler_fn(exceptions::exception_handler_noerror);
        idt.debug.set_handler_fn(exceptions::exception_handler_noerror);
        idt.non_maskable_interrupt.set_handler_fn(exceptions::exception_handler_noerror);
        idt.breakpoint.set_handler_fn(exceptions::exception_handler_noerror);
        idt.overflow.set_handler_fn(exceptions::exception_handler_noerror);
        idt.bound_range_exceeded.set_handler_fn(exceptions::exception_handler_noerror);
        idt.invalid_opcode.set_handler_fn(exceptions::exception_handler_noerror);
        idt.device_not_available.set_handler_fn(exceptions::exception_handler_noerror);
        idt.double_fault.set_handler_fn(exceptions::double_fault_handler);
        idt.invalid_tss.set_handler_fn(exceptions::exception_handler_error);
        idt.segment_not_present.set_handler_fn(exceptions::exception_handler_error);
        idt.stack_segment_fault.set_handler_fn(exceptions::exception_handler_error);
        idt.general_protection_fault.set_handler_fn(exceptions::exception_handler_error);
        idt.page_fault.set_handler_fn(exceptions::page_fault_handler);
        idt.x87_floating_point.set_handler_fn(exceptions::exception_handler_noerror);
        idt.alignment_check.set_handler_fn(exceptions::exception_handler_error);
        idt.simd_floating_point.set_handler_fn(exceptions::exception_handler_noerror);
        idt.virtualization.set_handler_fn(exceptions::exception_handler_noerror);
        idt.security_exception.set_handler_fn(exceptions::exception_handler_error);
        idt
    };
}

pub fn init() {
    IDT.load();
}

type X86handler = extern "x86-interrupt" fn(stack_frame: &mut InterruptStackFrame);

#[allow(dead_code)]
pub fn idt_load_handler(_index: usize, _handler: X86handler) {
    // IDT[index].set_handler_fn(handler);
}