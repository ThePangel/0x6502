use crate::bus::Bus;
pub struct Cpu6502 {
    pub a: u8,

    pub y: u8,

    pub x: u8,

    pub pc: u16,

    pub sp: u8,

    pub p: u8, 

   
}

impl Cpu6502 {
    pub fn new() -> Self {
        
        Self {
            a: 0,
            y: 0,
            x: 0,
            pc: 0,
            sp: 0,
            p: 0,
        }

    }
    pub fn reset<B: Bus>(&mut self, bus: &B) {
        
        let pch = bus.read(0xFFFD);
        let pcl = bus.read(0xFFFC);

        self.pc = u16::from_le_bytes([pcl, pch]);

    }

    pub fn step<B: Bus>(&mut self, bus: &B) {
        
    }
}