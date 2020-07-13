use crate::kernel;
use lazy_static::lazy_static;
use pc_keyboard::{Keyboard, ScancodeSet1, HandleControl, layouts, KeyCode, DecodedKey};
use spin::Mutex;
use x86_64::instructions::port::Port;
use crate::{print};

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode)
    );
}

pub fn init() {
    kernel::idt::register_irq_handler(1, keyboard_interrupt_handler);
}

fn read_scancode() -> u8 {
    let mut port = Port::new(0x60);
    unsafe {
        port.read()
    }
}

fn keyboard_interrupt_handler() {
    let mut keyboard = KEYBOARD.lock();
    let scancode = read_scancode();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            let c = match key {
                DecodedKey::Unicode(c) => c,
                DecodedKey::RawKey(KeyCode::ArrowLeft)  => '←',
                DecodedKey::RawKey(KeyCode::ArrowUp)    => '↑',
                DecodedKey::RawKey(KeyCode::ArrowRight) => '→',
                DecodedKey::RawKey(KeyCode::ArrowDown)  => '↓',
                DecodedKey::RawKey(_) => '\0'
            };
            print!("{}", c);
        }
    }
}
