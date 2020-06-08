use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const BUFFER_ADDR:   usize = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH:  usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(
        VgaWriter {
            posx: 0,
            posy: BUFFER_HEIGHT - 1,
            color_code: VgaColor::generate(VGAColors::White, VGAColors::Black),
            buffer: unsafe { 
                &mut *(BUFFER_ADDR as *mut Buffer)
            },
        }
    );
}

#[warn(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VGAColors {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct VgaColor(u8);

impl VgaColor {
    fn generate(foreground: VGAColors, background: VGAColors) -> VgaColor {
        VgaColor((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaPx {
    ascii_character: u8,
    color_code: VgaColor,
}

#[repr(transparent)]
struct Buffer {
    pxls: [[Volatile<VgaPx>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[allow(dead_code)]
pub struct VgaWriter {
    posx: usize,
    posy: usize,
    color_code: VgaColor,
    buffer: &'static mut Buffer,
}

impl VgaWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.scroll(),
            b'\t' => {
                self.posx = (self.posx + (8 as usize)) & !(7 as usize);
            },
            b'\r' => {
                self.posx = 0x0;
            },
            0x20..=0x7E => {
                if self.posx >= BUFFER_WIDTH {
                    self.scroll();
                }
                let color_code = self.color_code;
                self.buffer.pxls[BUFFER_HEIGHT - 1][self.posx].write(
                    VgaPx {
                        ascii_character: byte,
                        color_code,
                    }
                );
                self.posx += 1;
            },
            _ => {}
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            if byte <= 0x7E {
                self.write_byte(byte);
            }
        }
    }

    fn scroll(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.pxls[row][col].read();
                self.buffer.pxls[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.posx = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = VgaPx {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.pxls[row][col].write(blank);
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
