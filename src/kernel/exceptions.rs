use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use x86_64;

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

pub extern "x86-interrupt" fn exception_handler_noerror(stack_frame: &mut InterruptStackFrame) {
    // println!("kernel panic: {}.", EXCEPTIONS.get(int).unwrap_or(&DEFAULT));
    // hlt();
}

pub extern "x86-interrupt" fn exception_handler_error(stack_frame: &mut InterruptStackFrame, _error: u64) {
    // println!("kernel panic: {}.", EXCEPTIONS.get(int).unwrap_or(&DEFAULT));
    // hlt();
}

pub extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut InterruptStackFrame, pferr: PageFaultErrorCode) {

}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, _err: u64) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
