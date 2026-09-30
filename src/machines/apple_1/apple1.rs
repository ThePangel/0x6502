use std::time::Duration;

use crate::{app, cpu::cpu6502::Cpu6502, machines::apple_1::apple1Bus::Apple1Bus};

pub struct Apple1 {
    pub cpu: Cpu6502,
    pub bus: Apple1Bus,
    pub clock_speed: f64,
    pub remainding_cycles: f64,
}

impl Apple1 {
    pub fn new() -> Self {
        let mut apple1 = Apple1 {
            cpu: Cpu6502::new(),
            bus: Apple1Bus::new(),
            clock_speed: 1_023_000.0,
            remainding_cycles: 0.0,
        };
        apple1.cpu.reset(&mut apple1.bus);
        apple1  
    }
    pub fn reset(&mut self) {
        self.remainding_cycles = 0.0;
        self.bus = Apple1Bus::new();
        self.cpu.reset(&mut self.bus);
    }
    pub fn consume_cycles(&mut self, duration: Duration) {
        let due_cycles = (duration.as_secs_f64() * self.clock_speed) + self.remainding_cycles;

        let cycles = due_cycles.floor() as u64;

        self.remainding_cycles = due_cycles - cycles as f64;

        for _ in 0..cycles {
            self.cpu.cycle(&mut self.bus);
        }
    }
}
