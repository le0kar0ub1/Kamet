use crate::print;
use crate::drivers::vga::*;

static KAMET_ART_SIGNATURE: &str = "
    __ __ ___    __  _______________\r
   / //_//   |  /  |/  / ____/_  __/\r
  / ,<  / /| | / /|_/ / __/   / /   \r
 / /| |/ ___ |/ /  / / /___  / /    \r
/_/ |_/_/  |_/_/  /_/_____/ /_/     \r
";

pub fn menu() {
    WRITER.lock().fclear();
    WRITER.lock().set_cursor(0, 0);
    WRITER.lock().set_color(VGAColors::Yellow, VGAColors::Black);
    print!("{}\n", KAMET_ART_SIGNATURE);
    WRITER.lock().set_color(VGAColors::White, VGAColors::Black);
    print!("kernel init routine...\n");
    loop{};
}