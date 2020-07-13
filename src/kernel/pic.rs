use crate::*;
use pic8259_simple::ChainedPics;
use spin;
use x86_64::instructions::port::Port;

const INTERRUPT_INDEX_PIC1: u8 = 32;
const INTERRUPT_INDEX_PIC2: u8 = 40;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(INTERRUPT_INDEX_PIC1, INTERRUPT_INDEX_PIC2) });

pub fn irq_index(base: u8) -> u8 {
    base + INTERRUPT_INDEX_PIC1
}

#[allow(dead_code)]
pub fn set_irq_mask(irq: u8) {
    let mut port: Port<u8> = Port::new(if irq < 8 { INTERRUPT_INDEX_PIC1 as u16 } else { INTERRUPT_INDEX_PIC2 as u16 } );
    unsafe {
        let value = port.read() | (1 << (if irq < 8 { irq } else { irq - 8 }));
        port.write(value);
    }
}

#[allow(dead_code)]
pub fn clear_irq_mask(irq: u8) {
    let mut port: Port<u8> = Port::new(if irq < 8 { INTERRUPT_INDEX_PIC1 as u16 } else { INTERRUPT_INDEX_PIC2 as u16 } );
    unsafe {
        let value = port.read() & !(1 << if irq < 8 { irq } else { irq - 8 });
        port.write(value);
    }
}