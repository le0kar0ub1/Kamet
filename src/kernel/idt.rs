use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::kernel::exceptions;
use crate::kernel::pic;
use crate::kernel::gdt;

fn unhandled_irq() {}

macro_rules! irq_handler {
    ($handler:ident, $irq:expr) => {
        pub extern "x86-interrupt" fn $handler(_stack_frame: &mut InterruptStackFrame) {
            let handlers = IRQ_HANDLERS.lock();
            handlers[$irq]();
            unsafe { pic::PICS.lock().notify_end_of_interrupt(pic::irq_index($irq)); }
        }
    };
}

irq_handler!(irq0_handler, 0);
irq_handler!(irq1_handler, 1);
irq_handler!(irq2_handler, 2);
irq_handler!(irq3_handler, 3);
irq_handler!(irq4_handler, 4);
irq_handler!(irq5_handler, 5);
irq_handler!(irq6_handler, 6);
irq_handler!(irq7_handler, 7);
irq_handler!(irq8_handler, 8);
irq_handler!(irq9_handler, 9);
irq_handler!(irq10_handler, 10);
irq_handler!(irq11_handler, 11);
irq_handler!(irq12_handler, 12);
irq_handler!(irq13_handler, 13);
irq_handler!(irq14_handler, 14);
irq_handler!(irq15_handler, 15);

lazy_static! {
    pub static ref IRQ_HANDLERS: Mutex<[fn(); 16]> = Mutex::new([unhandled_irq; 16]);

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
        unsafe {
            idt.double_fault.set_handler_fn(exceptions::double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
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
        idt[33].set_handler_fn(pic::keyboard_handler);

        idt[pic::irq_index(0) as usize].set_handler_fn(irq0_handler);
        idt[pic::irq_index(1) as usize].set_handler_fn(irq1_handler);
        idt[pic::irq_index(2) as usize].set_handler_fn(irq2_handler);
        idt[pic::irq_index(3) as usize].set_handler_fn(irq3_handler);
        idt[pic::irq_index(4) as usize].set_handler_fn(irq4_handler);
        idt[pic::irq_index(5) as usize].set_handler_fn(irq5_handler);
        idt[pic::irq_index(6) as usize].set_handler_fn(irq6_handler);
        idt[pic::irq_index(7) as usize].set_handler_fn(irq7_handler);
        idt[pic::irq_index(8) as usize].set_handler_fn(irq8_handler);
        idt[pic::irq_index(9) as usize].set_handler_fn(irq9_handler);
        idt[pic::irq_index(10) as usize].set_handler_fn(irq10_handler);
        idt[pic::irq_index(11) as usize].set_handler_fn(irq11_handler);
        idt[pic::irq_index(12) as usize].set_handler_fn(irq12_handler);
        idt[pic::irq_index(13) as usize].set_handler_fn(irq13_handler);
        idt[pic::irq_index(14) as usize].set_handler_fn(irq14_handler);
        idt[pic::irq_index(15) as usize].set_handler_fn(irq15_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

// type X86handler = extern "x86-interrupt" fn(stack_frame: &mut InterruptStackFrame);