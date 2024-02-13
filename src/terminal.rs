
#[allow(dead_code)]
pub enum VgaColour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

fn vga_entry_color(fg: VgaColour, bg: VgaColour) -> u8 {
    fg as u8 | (bg as u8) << 4
}

fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

static VGA_WIDTH: usize = 80;
static VGA_HEIGHT: usize = 25;

pub struct Terminal {
    pub row: usize,
    pub column: usize,
    pub colour: u8,
    pub buffer: *mut u16,
}

impl Terminal {
    pub fn putc(&mut self, c: u8) {
        match c {
            b'\n' => self.newline(),
            _ => {
                unsafe {
                    let index = self.row * VGA_WIDTH + self.column;
                    *self.buffer.offset(index as isize) = vga_entry(c, self.colour);
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
                    *self.buffer.offset(index as isize) = vga_entry(' ' as u8, self.colour);
                }
            }
        }
    }

    pub fn set_colour(&mut self, fg: VgaColour, bg: VgaColour) {
        self.colour = vga_entry_color(fg, bg);
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.column = x;
        self.row = y;
    }

    pub fn clear_row(&mut self, row: usize) {
        for x in 0..VGA_WIDTH {
            unsafe {
                let index = row * VGA_WIDTH + x;
                *self.buffer.offset(index as isize) = vga_entry(' ' as u8, self.colour);
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
        self.colour = vga_entry_color(VgaColour::LightGrey, VgaColour::Black);
        self.buffer = 0xB8000 as *mut u16;
        self.clear();
    }
}