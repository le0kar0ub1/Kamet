use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        InterruptDescriptorTable::new()
    };
}

pub fn init() {
    IDT.load();
}
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    // println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}


// #[allow(dead_code)]
// pub fn idt_load_handler (index: usize, handler: X86handler) {
//     // IDT[index].set_handler_fn(handler);
// }