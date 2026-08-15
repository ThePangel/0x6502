use crate::{
    bus::Bus,
    cpu::{
        cpu6502::Cpu6502,
        instructions::{Addressing, Operand},
    },
};

pub fn resolve<B: Bus>(mode: Addressing, cpu: &mut Cpu6502, bus: &B) -> Operand {
    match mode {
        Addressing::Accumulator => Operand::Accumulator,
        Addressing::Immediate => {
            let operand = Operand::Address(cpu.pc);
            cpu.pc += 1;

            operand
        }
        Addressing::Absolute => {
            let adl = cpu.read_byte(bus, cpu.pc);
            let adh = cpu.read_byte(bus, cpu.pc + 1);
            cpu.pc += 2;

            Operand::Address(u16::from_le_bytes([adl, adh]))
        }
        Addressing::ZPage => {
            let operand = Operand::Address(cpu.read_byte(bus, cpu.pc) as u16);
            cpu.pc += 1;

            operand
        }
        Addressing::IndexedZPageX => {
            let operand = Operand::Address((cpu.read_byte(bus, cpu.pc) + cpu.x) as u16);
            cpu.pc += 1;

            operand
        }
        Addressing::IndexedZPageY => {
            let operand = Operand::Address((cpu.read_byte(bus, cpu.pc) + cpu.y) as u16);
            cpu.pc += 1;

            operand
        }
        Addressing::IndexedAbsoluteX => {
            let bal = cpu.read_byte(bus, cpu.pc);
            let bah = cpu.read_byte(bus, cpu.pc + 1);
            cpu.pc += 2;

            if bal.checked_add(cpu.x).is_none() {
                cpu.cycles += 1
            }

            Operand::Address(u16::from_le_bytes([bal, bah]).wrapping_add(cpu.x as u16))
        }
        Addressing::IndexedAbsoluteY => {
            let bal = cpu.read_byte(bus, cpu.pc);
            let bah = cpu.read_byte(bus, cpu.pc + 1);
            cpu.pc += 2;

            if bal.checked_add(cpu.y).is_none() {
                cpu.cycles += 1
            }
            Operand::Address(u16::from_le_bytes([bal, bah]).wrapping_add(cpu.y as u16))
        }
        Addressing::Implied => Operand::Implied,
        Addressing::Relative => {
            let effective_addr = cpu
                .pc
                .wrapping_add_signed(cpu.read_byte(bus, cpu.pc) as i16);

            cpu.pc += 1;

            Operand::Address(effective_addr)
        }
        Addressing::IndexedIndirect => {
            let z_page_addr = cpu.x.wrapping_add(cpu.read_byte(bus, cpu.pc));
            let bal = cpu.read_byte(bus, z_page_addr as u16);
            let bah = cpu.read_byte(bus, z_page_addr.wrapping_add(1) as u16);

            cpu.pc += 1;

            Operand::Address(u16::from_le_bytes([bal, bah]))
        }
        Addressing::IndirectIndexed => {
            let z_page_addr = cpu.read_byte(bus, cpu.pc);
            let bal = cpu.read_byte(bus, z_page_addr as u16);
            let bah = cpu.read_byte(bus, z_page_addr.wrapping_add(1) as u16);

            if bal.checked_add(cpu.y).is_none() {
                cpu.cycles += 1
            }

            cpu.pc += 1;

            Operand::Address(u16::from_le_bytes([bal, bah]).wrapping_add(cpu.y as u16))
        }
        Addressing::AbsoluteIndirect => {
            let bal = cpu.read_byte(bus, cpu.pc);
            let bah = cpu.read_byte(bus, cpu.pc + 1);
            let effective_adrr = u16::from_le_bytes([bal, bah]);

            // REMINDER THIS IS THE HARDWARE BUG DONT FORGET
            let high_addr = if bal == 0xFF {
                u16::from_le_bytes([0x00, bah])
            } else {
                effective_adrr + 1
            };

            cpu.pc += 2;
            Operand::Address(u16::from_le_bytes([
                cpu.read_byte(bus, effective_adrr),
                cpu.read_byte(bus, high_addr),
            ]))
        }
    }
}
