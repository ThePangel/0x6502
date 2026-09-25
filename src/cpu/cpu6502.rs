use ratatui::style::Stylize;

use crate::{
    bus::Bus,
    cpu::{
        self,
        addressing::resolve,
        flags::set_break,
        instructions::{
            Addressing::Implied, Instruction, Operand, Operation::NOP, get_instruction,
        },
        operations::run_operation,
    },
};

pub struct OpcodeState {
    pub current_opcode: Instruction,

    pub operand: Operand,

    pub opcode_cycle: u8,

    pub page_cross: bool,

    pub latch: u8,

    pub branch_taken: bool,
}

impl OpcodeState {
    pub fn new() -> Self {
        OpcodeState {
            current_opcode: Instruction {
                operation: NOP,
                addressing: Implied,
            },
            opcode_cycle: 0,
            page_cross: false,
            operand: Operand::Implied,
            latch: 0,
            branch_taken: false,
        }
    }
}
pub struct Cpu6502 {
    pub a: u8,

    pub y: u8,

    pub x: u8,

    pub pc: u16,

    pub sp: u8,

    pub p: u8,

    pub cycles: u64,

    pub opcode_state: OpcodeState,
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
            opcode_state: OpcodeState::new(),
        }
    }
    pub fn reset<B: Bus>(&mut self, bus: &mut B) {
        let pcl = self.read_byte(bus, 0xFFFC);
        let pch = self.read_byte(bus, 0xFFFD);

        self.pc = u16::from_le_bytes([pcl, pch]);
    }

    pub fn cycle<B: Bus>(&mut self, bus: &mut B) {
        if self.opcode_state.opcode_cycle == 0 {
            self.opcode_state = OpcodeState::new();
            let opcode = self.read_byte(bus, self.pc);
            self.pc = self.pc.wrapping_add(1);
            self.opcode_state.current_opcode = get_instruction(opcode);
            return;
        }
        resolve(self, bus);
    }

    pub fn read_byte<B: Bus>(&mut self, bus: &mut B, addr: u16) -> u8 {
        self.cycles = self.cycles.wrapping_add(1);
        self.opcode_state.opcode_cycle = self.opcode_state.opcode_cycle.wrapping_add(1);

        bus.read(addr)
    }

    pub fn write_byte<B: Bus>(&mut self, bus: &mut B, addr: u16, byte: u8) {
        self.cycles = self.cycles.wrapping_add(1);

        self.opcode_state.opcode_cycle = self.opcode_state.opcode_cycle.wrapping_add(1);

        bus.write(addr, byte);
    }
}
