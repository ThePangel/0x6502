use std::ptr::read;

use crate::{
    bus::Bus,
    cpu::{
        cpu6502::Cpu6502,
        instructions::{Addressing, Operand},
    },
};

pub fn resolve<B: Bus>(mode: Addressing, cpu: &mut Cpu6502, bus: &B) -> Operand {
    match mode {
        Addressing::Accumulator => Operand::None,
        Addressing::Immideate => {
            let operand = Operand::Value(bus.read(cpu.pc));
            cpu.pc += 1;

            operand
        }
        Addressing::Absolute => {
            let adl = bus.read(cpu.pc);
            let adh = bus.read(cpu.pc + 1);

            cpu.pc += 2;

            Operand::Address(u16::from_le_bytes([adl, adh]))
        }
        Addressing::ZPage => {
            let operand = Operand::Address(bus.read(cpu.pc) as u16);
            cpu.pc += 1;

            operand
        }
        Addressing::IndexedZPageX => {
            let operand = Operand::Address((bus.read(cpu.pc) + cpu.x) as u16);
            cpu.pc += 1;

            operand
        }
        Addressing::IndexedZPageY => {
            let operand = Operand::Address((bus.read(cpu.pc) + cpu.y) as u16);
            cpu.pc += 1;

            operand
        }
        Addressing::IndexedAnbsoluteX => {
            let bal = bus.read(cpu.pc);
            let bah = bus.read(cpu.pc + 1);
            cpu.pc += 2;

            Operand::Address(u16::from_le_bytes([bal, bah]).wrapping_add(cpu.x as u16))
        }
        Addressing::IndexedAnbsoluteY => {
            let bal = bus.read(cpu.pc);
            let bah = bus.read(cpu.pc + 1);
            cpu.pc += 2;

            Operand::Address(u16::from_le_bytes([bal, bah]).wrapping_add(cpu.y as u16))
        }
        Addressing::Implied => Operand::None,
        Addressing::Relative => {
            let operand = Operand::Address(cpu.pc.wrapping_add_signed(bus.read(cpu.pc) as i16));

            cpu.pc += 1;

            operand
        }
        Addressing::IndexedIndirect => {
            let z_page_addr = cpu.x.wrapping_add(bus.read(cpu.pc));
            let bal = bus.read(z_page_addr as u16);
            let bah = bus.read(z_page_addr.wrapping_add(1) as u16);

            cpu.pc += 1;

            Operand::Address(u16::from_le_bytes([bal, bah]))
        }
        Addressing::IndirectIndexed => {
            let z_page_addr = bus.read(cpu.pc);
            let bal = bus.read(z_page_addr as u16);
            let bah = bus.read(z_page_addr.wrapping_add(1) as u16);

            cpu.pc += 1;

            Operand::Address(u16::from_le_bytes([bal, bah]).wrapping_add(cpu.y as u16))
        }
        Addressing::AbsoluteIndirect => {
            let bal = bus.read(cpu.pc);
            let bah = bus.read(cpu.pc + 1);
            let effective_adrr = u16::from_le_bytes([bal, bah]);

            // REMINDER THIS IS THE HARDWARE BUG DONT FORGET
            let high_addr = if bal == 0xFF {
                u16::from_le_bytes([0x00, bah])
            } else {
                effective_adrr + 1
            };

            cpu.pc += 2;
            Operand::Address(u16::from_le_bytes([
                bus.read(effective_adrr),
                bus.read(high_addr),
            ]))
        }
    }
}
