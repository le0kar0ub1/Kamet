use crate::*;
use pic8259_simple::ChainedPics;
use lazy_static::lazy_static;
use spin;
use x86_64::structures::idt::InterruptStackFrame;

const INTERRUPT_INDEX_PIC1: u8 = 32;
const INTERRUPT_INDEX_PIC2: u8 = 40;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(INTERRUPT_INDEX_PIC1, INTERRUPT_INDEX_PIC2) });

pub fn irq_index(base: u8) -> u8 {
    base + 32
}

pub extern "x86-interrupt" fn timer_handler(_stackframe: &mut InterruptStackFrame) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(32);
    }
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { 
        port.read() 
    };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
}