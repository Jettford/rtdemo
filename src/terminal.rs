#[path = "display.rs"]
mod display;

use self::display::{VgaColour, VgaEntry, VgaColourCode, VGA_HEIGHT, VGA_WIDTH};

pub struct Terminal {
    pub row: usize,
    pub column: usize,
    pub colour: VgaColour,
    pub buffer: *mut u16,
}

impl Terminal {
    pub fn putc(&mut self, c: u8) {
        match c {
            b'\n' => self.newline(),
            _ => {
                unsafe {
                    let index = self.row * VGA_WIDTH + self.column;
                    *self.buffer.offset(index as isize) = VgaEntry::new(c, self.colour).to_u16();
                }
                self.column += 1;
            }
        }
    }

    fn newline(&mut self) {
        self.column = 0;
        self.row += 1;
    }

    pub fn write(&mut self, s: &str) {
        for c in s.bytes() {
            self.putc(c);
        }
    }

    pub fn clear(&mut self) {
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                unsafe {
                    let index = y * VGA_WIDTH + x;
                    *self.buffer.offset(index as isize) = VgaEntry::new(' ' as u8, self.colour).to_u16();
                }
            }
        }
    }

    pub fn set_colour(&mut self, fg: VgaColourCode, bg: VgaColourCode) {
        self.colour = VgaColour::new(fg, bg);
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.column = x;
        self.row = y;
    }

    pub fn clear_row(&mut self, row: usize) {
        for x in 0..VGA_WIDTH {
            unsafe {
                let index = row * VGA_WIDTH + x;
                *self.buffer.offset(index as isize) = VgaEntry::new(' ' as u8, self.colour).to_u16();
            }
        }
    }

    pub fn scroll(&mut self) {
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                unsafe {
                    let index = (y - 1) * VGA_WIDTH + x;
                    let index2 = y * VGA_WIDTH + x;
                    let value = *self.buffer.offset(index2 as isize);
                    *self.buffer.offset(index as isize) = value;
                }
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
    }
    
    pub fn writeln(&mut self, s: &str) {
        self.write(s);
        self.newline();
    }

    pub fn init(&mut self) {
        self.row = 0;
        self.column = 0;
        self.colour = VgaColour::new(VgaColourCode::LightGrey, VgaColourCode::Black);
        self.buffer = 0xB8000 as *mut u16;

        self.clear();
    }
}

impl Default for Terminal {
    fn default() -> Terminal {
        Terminal {
            row: 0,
            column: 0,
            colour: VgaColour::new(VgaColourCode::LightGrey, VgaColourCode::Black),
            buffer: 0 as *mut u16,
        }
    }
}