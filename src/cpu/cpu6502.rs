use std::ptr::read;

use crate::bus::Bus;
pub struct Cpu6502 {
    pub a: u8,

    pub y: u8,

    pub x: u8,

    pub pc: u16,

    pub sp: u8,

    pub p: u8,

    pub cycles: u64,
}

impl Cpu6502 {
    pub fn new() -> Self {
        Self {
            a: 0,
            y: 0,
            x: 0,
            pc: 0,
            sp: 0xFF,
            p: 0,
            cycles: 0,
        }
    }
    pub fn reset<B: Bus>(&mut self, bus: &B) {
        let pch = self.read_byte(bus, 0xFFFD);
        let pcl = self.read_byte(bus, 0xFFFC);

        self.pc = u16::from_le_bytes([pcl, pch]);
    }

    pub fn step<B: Bus>(&mut self, bus: &B) {
        let opcode = self.read_byte(bus, self.pc);
        self.pc += 1;
        self.cycles += 1;
    }

    pub fn read_byte<B: Bus>(&mut self, bus: &B, addr: u16) -> u8 {
        self.cycles += 1;
        bus.read(addr)
    }

    pub fn write_byte<B: Bus>(&mut self, bus: &B, addr: u16, byte: u8) {
        self.cycles += 1;
        bus.write(addr, byte);
    }
}
