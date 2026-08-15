use std::path::Prefix::VerbatimDisk;

use crate::{
    bus::Bus,
    cpu::{
        addressing,
        cpu6502::Cpu6502,
        flags::{
            self, get_carry, get_decimal, get_negative, get_zero, set_break, set_carry,
            set_interrupt_disable, set_negative, set_overflow, set_zero,
        },
        instructions::{
            Operand::{self, Address},
            Operation::{self, ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK},
        },
    },
};

fn run_operation<B: Bus>(operation: Operation, operand: Operand, cpu: &mut Cpu6502, bus: &B) {
    let (value, addr) = match operand {
        Operand::Address(addr) => (Some(cpu.read_byte(bus, addr)), Some(addr)),
        Operand::Accumulator => (Some(cpu.a), None),
        Operand::Implied => (None, None),
    };

    match operation {
        ADC => {
            let value = value.expect("ADC requires a value operand");

            let (sum1, overflow1) = cpu.a.overflowing_add(value);
            let (final_sum, overflow2) = sum1.overflowing_add(get_carry(cpu) as u8);

            let signed_overflow = !(cpu.a ^ value) & (cpu.a ^ final_sum) & 0x80;

            set_overflow(cpu, signed_overflow != 0);
            set_zero(cpu, final_sum == 0);
            set_negative(cpu, (final_sum & 0x80) != 0);

            if get_decimal(cpu) {
                let mut low = (cpu.a & 0x0F) + (value & 0x0F) + get_carry(cpu) as u8;
                let mut high = (cpu.a >> 4) + (value >> 4);

                if low > 0x09 {
                    low += 0x06;
                }
                if low > 0x0F {
                    high += 1;
                    low &= 0x0F;
                }

                if high > 0x09 {
                    high += 0x06;
                }
                set_carry(cpu, high > 0x0F);

                cpu.a = ((high << 4) & 0xF0) | (low & 0x0F)
            } else {
                set_carry(cpu, overflow1 || overflow2);
                cpu.a = final_sum
            }
        }
        AND => {
            let value = value.expect("AND requires a value operand");

            cpu.a &= value;

            set_zero(cpu, cpu.a == 0);
            set_negative(cpu, (cpu.a & 0x80) != 0);
        }
        ASL => {
            let mut value = value.expect("ASL requires a value operand");

            set_carry(cpu, value & 0x80 != 0);

            value <<= 1;

            match addr {
                Some(addr) => cpu.write_byte(bus, addr, value),
                None => cpu.a = value,
            }

            set_zero(cpu, value == 0);
            set_negative(cpu, (value & 0x80) != 0);
        }
        BCC => {
            let addr = addr.expect("BCC requires a address operand");

            if !get_carry(cpu) {
                if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    cpu.cycles += 1
                };
                cpu.pc = addr;
                cpu.cycles += 1;
            }
        }
        BCS => {
            let addr = addr.expect("BCC requires a address operand");

            if get_carry(cpu) {
                if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    cpu.cycles += 1
                };
                cpu.pc = addr;
                cpu.cycles += 1;
            }
        }
        BEQ => {
            let addr = addr.expect("BCC requires a address operand");

            if get_zero(cpu) {
                if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    cpu.cycles += 1
                };
                cpu.pc = addr;
                cpu.cycles += 1;
            }
        }
        BIT => {
            let value = value.expect("BIT REQUIRESA VALUE OPERAND");

            let tested_byte = cpu.a & value;

            set_zero(cpu, tested_byte == 0);
            set_negative(cpu, (value >> 7) & 0x01 != 0);
            set_overflow(cpu, (value >> 6) & 0x01 != 0);
        }
        BMI => {
            let addr = addr.expect("BMI requires a address operand");

            if get_negative(cpu) {
                if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    cpu.cycles += 1
                };
                cpu.pc = addr;
                cpu.cycles += 1;
            }
        }
        BNE => {
            let addr = addr.expect("BNE requires a address operand");

            if !get_zero(cpu) {
                if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    cpu.cycles += 1
                };
                cpu.pc = addr;
                cpu.cycles += 1;
            }
        }
        BPL => {
            let addr = addr.expect("BPL requires a address operand");

            if !get_negative(cpu) {
                if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    cpu.cycles += 1
                };
                cpu.pc = addr;
                cpu.cycles += 1;
            }
        }
        BRK => {
            cpu.pc += 1;

            cpu.write_byte(bus, 0x0100 + cpu.sp as u16, (cpu.pc >> 8) as u8);
            cpu.sp -= 1;
            cpu.write_byte(bus, 0x0100 + cpu.sp as u16, (cpu.pc & 0xFF) as u8);
            cpu.sp -= 1;

            cpu.write_byte(bus, 0x0100 + cpu.sp as u16, set_break(cpu.p));
            cpu.sp -= 1;

            set_interrupt_disable(cpu, true);

            let adl = cpu.read_byte(bus, 0xFFFE);
            let adh = cpu.read_byte(bus, 0xFFFF);

            cpu.pc = u16::from_le_bytes([adl, adh]);
        }

        _ => todo!("temp wildcard"),
    }
}
