use std::arch::asm;

pub struct Port {
    port: u16,
}

impl Port {
    pub fn new(port: u16) -> Port {
        Port { port: port }
    }

    pub fn read(&self) -> u8 {
        let result: u8;
        unsafe {
            asm!(
                "in %dx, %al",
                "mov {}, %al",
                out(reg_byte) result,
            );
        }
        result
    }

    pub fn write(&self, data: u8) {
        unsafe {
            asm!(
                "out %al, %dx",
                in("al") data,
            );
        }
    }

    pub fn read32(&self) -> u32 {
        let result: u32;
        unsafe {
            asm!(
                "in %dx, %eax",
                "mov {}, %eax",
                out(reg) result,
            );
        }
        result
    }

    pub fn write32(&self, data: u32) {
        unsafe {
            asm!(
                "out %eax, %dx",
                in("eax") data,
            );
        }
    }
}