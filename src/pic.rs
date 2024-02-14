#[path = "io.rs"]
mod io;

use io::Port;

pub struct Pic {
    pub pic1_port: Port,
    pub pic2_port: Port,

    pub pic1_data: Port,
    pub pic2_data: Port,
}

impl Pic {
    pub const PIC_ONE: u8 = 0x20;
    pub const PIC_TWO: u8 = 0xA0;
    pub const PIC_ONE_COMMAND: u8 = 0x20;
    pub const PIC_ONE_DATA: u8 = 0x21;
    pub const PIC_TWO_COMMAND: u8 = 0xA0;
    pub const PIC_TWO_DATA: u8 = 0xA1;
    pub const PIC_END_OF_INTERRUPT: u8 = 0x20;

    pub const ICW1: u8 = 0x11;
    pub const ICW4: u8 = 0x01;

    pub fn new() -> Pic {
        Pic {
            pic1_port: Port::new(Pic::PIC_ONE.into()),
            pic2_port: Port::new(Pic::PIC_TWO.into()),
            pic1_data: Port::new(Pic::PIC_ONE_DATA.into()),
            pic2_data: Port::new(Pic::PIC_TWO_DATA.into()),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let a1 = self.pic1_port.read();
            let a2 = self.pic2_port.read();

            self.pic1_port.write(Pic::ICW1);
            self.pic2_port.write(Pic::ICW1);

            self.pic1_data.write(0x20);
            self.pic2_data.write(0x28);

            self.pic1_data.write(4);
            self.pic2_data.write(2);

            self.pic1_data.write(Pic::ICW4);
            self.pic2_data.write(Pic::ICW4);

            self.pic1_port.write(a1);
            self.pic2_port.write(a2);
        }
    }

    pub fn end_of_interrupt(&mut self, _irq: u8) {
        unsafe {
            if _irq >= 0x28 {
                self.pic2_port.write(Pic::PIC_END_OF_INTERRUPT);
            }
            
            self.pic1_port.write(Pic::PIC_END_OF_INTERRUPT);
        }
    }
}


