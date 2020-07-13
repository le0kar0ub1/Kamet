#![allow(dead_code)]

use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use x86_64;

use crate::*;

static EXCEPTIONS: &[&str] = &[
    "Division By Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint",
    "Into Detected Overflow",
    "Out of Bounds",
    "Invalid Opcode",
    "No Coprocessor (not available)",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Bad TSS",
    "Segment Not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault",
    "Unknown Interrupt (reserved)",
    "Coprocessor (FPE) Fault",
    "Alignment Check",
    "Machine Check",
    "SIMD (FPE)",
    "Virtualization exception",
];
static DEFAULT: &str = "Unhandled interrupt";

pub extern "x86-interrupt" fn division_by_zero_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: division_by_zero\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn debug_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: debug\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn non_maskable_interrupt_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: non_maskable_interrupt\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn breakpoint_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: breakpoint\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn overflow_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: overflow\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn bound_range_exceeded_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: bound_range_exceeded\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn invalid_opcode_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: invalid_opcode\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn device_not_available_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: device_not_available\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn double_fault_handler(stackframe: &mut InterruptStackFrame, _err: u64) -> ! {
    println!("Exception taken: double_fault\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn invalid_tss_handler(stackframe: &mut InterruptStackFrame, _err: u64) {
    println!("Exception taken: invalid_tss\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn segment_not_present_handler(stackframe: &mut InterruptStackFrame, _err: u64) {
    println!("Exception taken: segment_not_present\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn stack_segment_fault_handler(stackframe: &mut InterruptStackFrame, _err: u64) {
    println!("Exception taken: stack_segment_fault\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn general_protection_fault_handler(stackframe: &mut InterruptStackFrame, _err: u64) {
    println!("Exception taken: general_protection_fault\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn page_fault_handler(stackframe: &mut InterruptStackFrame, err: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    println!("Exception taken: page_fault\n{:?}", stackframe);
    println!("Error Code: {:?}", err);
    println!("Accessed Address: {:?}", Cr2::read());
    println!("{:#?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn x87_floating_point_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: x87_floating_point\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn alignment_check_handler(stackframe: &mut InterruptStackFrame, _err: u64) {
    println!("Exception taken: alignment_check\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn simd_floating_point_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: simd_floating_point\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn virtualization_handler(stackframe: &mut InterruptStackFrame) {
    println!("Exception taken: virtualization\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}
pub extern "x86-interrupt" fn security_exception_handler(stackframe: &mut InterruptStackFrame, _err: u64) {
    println!("Exception taken: security_exception\n{:?}", stackframe);
    loop { x86_64::instructions::hlt() };
}