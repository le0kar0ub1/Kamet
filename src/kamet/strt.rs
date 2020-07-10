use crate::*;
use crate::drivers::vga::*;

pub fn menu() {
    WRITER.lock().fclear();
    WRITER.lock().set_cursor((BUFFER_WIDTH / 2) - 8, BUFFER_HEIGHT / 4);
    WRITER.lock().set_color(VGAColors::Blue, VGAColors::Black);
    print!("Wecome to Kamet!");
    loop{};
}