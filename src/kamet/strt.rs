use crate::*;
use crate::drivers::vga::*;

static KAMET_ART_SIGNATURE: &str = "
 ____  __.                      __\r
|    |/ _|____    _____   _____/  |_\r
|      < \\__  \\  /     \\_/ __ \\   __\\\r
|    |  \\ / __ \\|  Y Y  \\  ___/|  |\r
|____|__ (____  /__|_|  /\\___  >__|\r
        \\/    \\/      \\/     \\/\r
";

pub fn menu() {
    WRITER.lock().fclear();
    WRITER.lock().set_cursor(0, 0);
    WRITER.lock().set_color(VGAColors::LightGreen, VGAColors::Black);
    print!("{}\nkernel init routine...", KAMET_ART_SIGNATURE);
    loop{};
}