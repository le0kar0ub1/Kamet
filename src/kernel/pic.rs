use crate::*;
use pic8259_simple::ChainedPics;
use spin;
use x86_64::structures::idt::InterruptStackFrame;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(32, 40) });

pub extern "x86-interrupt" fn timer_handler(_stackframe: &mut InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(0);
    }
}