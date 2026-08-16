use crate::{app, cpu::cpu6502::Cpu6502, machines::apple_1::apple1Bus::Apple1Bus};

pub struct Apple1 {
    pub cpu: Cpu6502,
    pub bus: Apple1Bus,
    pub clock_speed: f64,
}

impl Apple1 {
    pub fn new() -> Self {
        let mut apple1 = Apple1 {
            cpu: Cpu6502::new(),
            bus: Apple1Bus::new(),
            clock_speed: 1_023_000.0,
        };
        apple1.cpu.reset(&mut apple1.bus);
        apple1
    }
    pub fn reset(&mut self) {
        self.bus = Apple1Bus::new();
        self.cpu.reset(&mut self.bus);
    }
}
