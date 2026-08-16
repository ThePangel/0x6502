use crate::{
    bus::Bus,
    cpu::{
        cpu6502::Cpu6502,
        flags::{
            get_carry, get_decimal, get_negative, get_overflow, get_zero, set_break, set_carry,
            set_decimal, set_interrupt_disable, set_negative, set_overflow, set_zero,
        },
        instructions::{
            Operand::{self},
            Operation::{
                self, ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC, CLD,
                CLI, CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX,
                LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI,
                STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,
            },
        },
    },
};

fn run_operation<B: Bus>(operation: Operation, operand: Operand, cpu: &mut Cpu6502, bus: &B) {
    let (accumulator, addr) = match operand {
        Operand::Address(addr) => (None, Some(addr)),
        Operand::Accumulator => (Some(cpu.a), None),
        Operand::Implied => (None, None),
    };

    match operation {
        ADC => {
            let value = cpu.read_byte(bus, addr.expect("ADC requires an address operand"));

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
            let value = cpu.read_byte(bus, addr.expect("AND requires an address operand"));

            cpu.a &= value;

            set_zero(cpu, cpu.a == 0);
            set_negative(cpu, (cpu.a & 0x80) != 0);
        }
        ASL => {
            let mut value = match accumulator {
                Some(accumulator) => accumulator,
                None => cpu.read_byte(bus, addr.expect("ASL requires an address operand")),
            };

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
            let value = cpu.read_byte(bus, addr.expect("BIT requires an address operand"));

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
        BVC => {
            let addr = addr.expect("BVC requires a address operand");

            if !get_overflow(cpu) {
                if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    cpu.cycles += 1
                };
                cpu.pc = addr;
                cpu.cycles += 1;
            }
        }
        BVS => {
            let addr = addr.expect("BVC requires a address operand");

            if get_overflow(cpu) {
                if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    cpu.cycles += 1
                };
                cpu.pc = addr;
                cpu.cycles += 1;
            }
        }
        CLC => {
            set_carry(cpu, false);
        }
        CLD => {
            set_decimal(cpu, false);
        }
        CLI => {
            set_interrupt_disable(cpu, false);
        }
        CLV => {
            set_overflow(cpu, false);
        }
        CMP => {
            let value = cpu.read_byte(bus, addr.expect("CMP requires an address operand"));

            let compared_value = cpu.a.wrapping_sub(value);

            set_carry(cpu, cpu.a >= value);
            set_zero(cpu, cpu.a == value);
            set_negative(cpu, (compared_value >> 7) & 0x01 != 0);
        }
        CPX => {
            let value = cpu.read_byte(bus, addr.expect("CPX requires an address operand"));

            let compared_value = cpu.x.wrapping_sub(value);

            set_carry(cpu, cpu.x >= value);
            set_zero(cpu, cpu.x == value);
            set_negative(cpu, (compared_value >> 7) & 0x01 != 0);
        }
        CPY => {
            let value = cpu.read_byte(bus, addr.expect("CPY requires an address operand"));

            let compared_value = cpu.y.wrapping_sub(value);

            set_carry(cpu, cpu.y >= value);
            set_zero(cpu, cpu.y == value);
            set_negative(cpu, (compared_value >> 7) & 0x01 != 0);
        }
        DEC => {
            let value = cpu.read_byte(bus, addr.expect("DEC requires an address operand"));
            let addr = addr.expect("DEC requires an address operand");

            let decremented_value = value.wrapping_sub(1);

            cpu.cycles += 1;
            cpu.write_byte(bus, addr, decremented_value);

            set_zero(cpu, decremented_value == 0);
            set_negative(cpu, (decremented_value >> 7) & 0x01 != 0);
        }
        DEX => {
            let decremented_value = cpu.x.wrapping_sub(1);

            cpu.x = decremented_value;

            set_zero(cpu, decremented_value == 0);
            set_negative(cpu, (decremented_value >> 7) & 0x01 != 0);
        }
        DEY => {
            let decremented_value = cpu.y.wrapping_sub(1);

            cpu.y = decremented_value;

            set_zero(cpu, decremented_value == 0);
            set_negative(cpu, (decremented_value >> 7) & 0x01 != 0);
        }
        EOR => {
            let value = cpu.read_byte(bus, addr.expect("EOR requires an address operand"));

            let xor_value = value ^ cpu.a;

            cpu.a = xor_value;

            set_zero(cpu, xor_value == 0);
            set_negative(cpu, (xor_value >> 7) & 0x01 != 0);
        }
        INC => {
            let value = cpu.read_byte(bus, addr.expect("INC requires an address operand"));
            let addr = addr.expect("DEC requires an address operand");

            let incremented_value = value.wrapping_add(1);

            cpu.cycles += 1;
            cpu.write_byte(bus, addr, incremented_value);

            set_zero(cpu, incremented_value == 0);
            set_negative(cpu, (incremented_value >> 7) & 0x01 != 0);
        }
        INX => {
            let incremented_value = cpu.x.wrapping_add(1);

            cpu.x = incremented_value;

            set_zero(cpu, incremented_value == 0);
            set_negative(cpu, (incremented_value >> 7) & 0x01 != 0);
        }
        INY => {
            let incremented_value = cpu.y.wrapping_add(1);

            cpu.y = incremented_value;

            set_zero(cpu, incremented_value == 0);
            set_negative(cpu, (incremented_value >> 7) & 0x01 != 0);
        }
        JMP => {
            let addr = addr.expect("JMP requires an address operand");
            cpu.pc = addr
        }
        JSR => {
            let addr = addr.expect("JSR requires a address operand");

            cpu.cycles += 1;
            cpu.write_byte(
                bus,
                0x0100 + cpu.sp as u16,
                (cpu.pc.wrapping_sub(1) >> 8) as u8,
            );
            cpu.sp -= 1;
            cpu.write_byte(
                bus,
                0x0100 + cpu.sp as u16,
                (cpu.pc.wrapping_sub(1) & 0xFF) as u8,
            );
            cpu.sp -= 1;

            cpu.pc = addr
        }
        LDA => {
            let value = cpu.read_byte(bus, addr.expect("LDA requires an address operand"));

            cpu.a = value;

            set_zero(cpu, value == 0);
            set_negative(cpu, (value >> 7) & 0x01 != 0);
        }
        LDX => {
            let value = cpu.read_byte(bus, addr.expect("LDA requires an address operand"));

            cpu.x = value;

            set_zero(cpu, value == 0);
            set_negative(cpu, (value >> 7) & 0x01 != 0);
        }
        LDY => {
            let value = cpu.read_byte(bus, addr.expect("LDA requires an address operand"));

            cpu.y = value;

            set_zero(cpu, value == 0);
            set_negative(cpu, (value >> 7) & 0x01 != 0);
        }
        LSR => {
            let mut value = match accumulator {
                Some(accumulator) => accumulator,
                None => cpu.read_byte(bus, addr.expect("LSR requires an address operand")),
            };

            set_carry(cpu, value & 0x80 != 0);

            value >>= 1;

            match addr {
                Some(addr) => cpu.write_byte(bus, addr, value),
                None => cpu.a = value,
            }

            set_zero(cpu, value == 0);
            set_negative(cpu, (value & 0x80) != 0);
        }
        NOP => {}
        ORA => {
            let value = cpu.read_byte(bus, addr.expect("ORA requires an address operand"));

            cpu.a ^= value;

            set_zero(cpu, cpu.a == 0);
            set_negative(cpu, (cpu.a & 0x80) != 0);
        }
        PHA => {
            cpu.cycles += 1;
            cpu.write_byte(bus, 0x0100 + cpu.sp as u16, cpu.a);
            cpu.sp -= 1;
        }
        PHP => {
            cpu.cycles += 1;
            cpu.write_byte(bus, 0x0100 + cpu.sp as u16, cpu.p);
            cpu.sp -= 1;
        }
        PLA => {
            cpu.cycles += 2;
            cpu.sp += 1;
            cpu.a = cpu.read_byte(bus, 0x0100 + cpu.sp as u16);
        }
        PLP => {
            cpu.cycles += 2;
            cpu.sp += 1;
            cpu.p = cpu.read_byte(bus, 0x0100 + cpu.sp as u16) & !0x10;
        }
        ROL => {
            let mut value = match accumulator {
                Some(accumulator) => accumulator,
                None => cpu.read_byte(bus, addr.expect("ROL requires an address operand")),
            };
            let old_carry = get_carry(cpu) as u8;

            set_carry(cpu, value & 0x80 != 0);

            value = (value << 1) | old_carry;

            match addr {
                Some(addr) => cpu.write_byte(bus, addr, value),
                None => cpu.a = value,
            }

            set_zero(cpu, value == 0);
            set_negative(cpu, (value & 0x80) != 0);
        }
        ROR => {
            let mut value = match accumulator {
                Some(accumulator) => accumulator,
                None => cpu.read_byte(bus, addr.expect("ROR requires an address operand")),
            };
            let old_carry = get_carry(cpu) as u8;

            set_carry(cpu, value & 0x01 != 0);

            value = (value >> 1) | (old_carry << 7);

            match addr {
                Some(addr) => cpu.write_byte(bus, addr, value),
                None => cpu.a = value,
            }

            set_zero(cpu, value == 0);
            set_negative(cpu, (value & 0x80) != 0);
        }
        RTI => {
            cpu.cycles += 2;

            cpu.sp += 1;
            cpu.p = cpu.read_byte(bus, 0x0100 + cpu.sp as u16) & !0x10;
            cpu.sp += 1;
            let pcl = cpu.read_byte(bus, 0x0100 + cpu.sp as u16);
            cpu.sp += 1;
            let pch = cpu.read_byte(bus, 0x0100 + cpu.sp as u16);
            cpu.pc = u16::from_le_bytes([pcl, pch]);
        }
        RTS => {
            cpu.cycles += 2;

            cpu.sp += 1;
            let pcl = cpu.read_byte(bus, 0x0100 + cpu.sp as u16);
            cpu.sp += 1;
            let pch = cpu.read_byte(bus, 0x0100 + cpu.sp as u16);
            cpu.cycles += 1;
            cpu.pc = u16::from_le_bytes([pcl, pch]) + 1;
        }
        SBC => {
            let value = cpu.read_byte(bus, addr.expect("SBC requires an adress operand"));

            let (sub1, overflow1) = cpu.a.overflowing_sub(value);
            let (final_sub, overflow2) = sub1.overflowing_sub(!get_carry(cpu) as u8);

            let signed_overflow = (cpu.a ^ final_sub) & (cpu.a ^ value) & 0x80;

            set_carry(cpu, !(overflow1 || overflow2));
            cpu.a = final_sub;

            set_overflow(cpu, signed_overflow != 0);
            set_zero(cpu, cpu.a == 0);
            set_negative(cpu, (cpu.a & 0x80) != 0);
        }
        SEC => {
            cpu.cycles += 1;
            set_carry(cpu, true);
        }
        SED => {
            cpu.cycles += 1;
            set_decimal(cpu, true);
        }
        SEI => {
            cpu.cycles += 1;
            set_interrupt_disable(cpu, true);
        }
        STA => {
            cpu.write_byte(bus, addr.expect("STA requires an address operand"), cpu.a);
        }
        STX => {
            cpu.write_byte(bus, addr.expect("STX requires an address operand"), cpu.x);
        }
        STY => {
            cpu.write_byte(bus, addr.expect("STY requires an address operand"), cpu.y);
        }
        TAX => {
            cpu.cycles += 1;

            cpu.x = cpu.a;

            set_zero(cpu, cpu.x == 0);
            set_negative(cpu, ((cpu.x >> 7) & 0x01) != 0);
        }
        TAY => {
            cpu.cycles += 1;

            cpu.y = cpu.a;

            set_zero(cpu, cpu.y == 0);
            set_negative(cpu, ((cpu.y >> 7) & 0x01) != 0);
        }
        TSX => {
            cpu.cycles += 1;

            cpu.x = cpu.sp;

            set_zero(cpu, cpu.x == 0);
            set_negative(cpu, ((cpu.x >> 7) & 0x01) != 0);
        }
        TXA => {
            cpu.cycles += 1;

            cpu.a = cpu.x;

            set_zero(cpu, cpu.a == 0);
            set_negative(cpu, ((cpu.a >> 7) & 0x01) != 0);
        }
        TXS => {
            cpu.cycles += 1;

            cpu.sp = cpu.x;
        }
        TYA => {
            cpu.cycles += 1;

            cpu.a = cpu.y;

            set_zero(cpu, cpu.a == 0);
            set_negative(cpu, ((cpu.a >> 7) & 0x01) != 0);
        }
    }
}
