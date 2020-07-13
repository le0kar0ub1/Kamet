use crate::*;
use core::sync::atomic::{AtomicUsize, Ordering};
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;

// const PIT_FREQUENCY: f64 = 1_193_181.666;
// const PIT_INTERVAL: f64 = (PIT_DIVIDER as f64) / PIT_FREQUENCY;
const PIT_DIVIDER: usize = 1193;
static PIT_TICKS: AtomicUsize = AtomicUsize::new(0);

fn set_pit_frequency(divider: u16) {
    interrupts::without_interrupts(|| {
        let bytes = divider.to_le_bytes();
        let mut cmd: Port<u8> = Port::new(0x43);
        let mut data: Port<u8> = Port::new(0x40);
        unsafe {
            cmd.write(0x36);
            data.write(bytes[0]);
            data.write(bytes[1]);
        }
    });
}

pub fn init() {
    set_pit_frequency(PIT_DIVIDER as u16);
    kernel::idt::register_irq_handler(0, pit_interrupt_handler);
}

fn pit_interrupt_handler() {
    PIT_TICKS.fetch_add(1, Ordering::Relaxed);
}