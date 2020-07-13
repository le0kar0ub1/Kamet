#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use x86_64;
use bootloader;

mod drivers;
use drivers::vga;

mod kamet;

mod kernel;

pub fn init() {
    kernel::gdt::init();
    kernel::idt::init();
    drivers::keyboard::init();
    drivers::serial::init();
    unsafe { 
        kernel::pic::PICS.lock().initialize()
    };
    x86_64::instructions::interrupts::enable();
    kamet::strt::menu();
}

bootloader::entry_point!(_start);

fn _start(_boot_info: &'static bootloader::BootInfo) -> ! {
    init();
    println!("All comes good");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panicked: {}", info);
    loop {}
}
