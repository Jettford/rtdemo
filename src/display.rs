
#[allow(dead_code)]
#[repr(u8)]
pub enum VgaColourCode {
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

pub static VGA_WIDTH: usize = 80;
pub static VGA_HEIGHT: usize = 25;

#[derive(Copy, Clone)]
pub struct VgaColour {
    pub foreground: u8,
    pub background: u8,
}

impl VgaColour {
    pub fn new(foreground: VgaColourCode, background: VgaColourCode) -> VgaColour {
        VgaColour { 
            foreground: foreground as u8,
            background: background as u8
        }
    }

    pub fn to_u8(&self) -> u8 {
        self.foreground | (self.background) << 4
    }
}

#[derive(Copy, Clone)]
pub struct VgaEntry {
    pub character: u8,
    pub colour: u8,
}

impl VgaEntry {
    pub fn new(character: u8, colour: VgaColour) -> VgaEntry {
        VgaEntry { 
            character: character,
            colour: colour.to_u8()
        }
    }

    pub fn to_u16(&self) -> u16 {
        self.character as u16 | (self.colour as u16) << 8
    }
}