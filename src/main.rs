#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;
use drivers::vga;

mod descriptors;

pub fn init() {
    descriptors::gdt::init();
    descriptors::idt::init();
    // unsafe { interrupts::PICS.lock().initialize() };
    // x86_64::instructions::interrupts::enable();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    println!("All comes good");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panicked: {}", info);
    loop {}
}
