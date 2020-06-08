#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

mod drivers;
use drivers::vga_buffer;
use drivers::serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    serial_println!("Hello World !",);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panicked: {}", info);
    loop {}
}
