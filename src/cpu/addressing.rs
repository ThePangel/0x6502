use crate::{
    bus::Bus,
    cpu::{
        cpu6502::Cpu6502,
        instructions::{
            Addressing,
            Operand::{self, Address},
            Operation::{STA, STX, STY},
        },
        operations::run_operation,
    },
};

pub fn resolve<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) {
    match cpu.opcode_state.current_opcode.addressing {
        Addressing::Accumulator => {
            cpu.read_byte(bus, cpu.pc);
            cpu.opcode_state.operand = Operand::Accumulator;
            run_operation(cpu, bus);
        }
        Addressing::Immediate => {
            cpu.opcode_state.operand = Operand::Address(cpu.pc);
            cpu.pc = cpu.pc.wrapping_add(1);
            run_operation(cpu, bus);
        }
        Addressing::Absolute => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            2 => {
                let adh = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);

                cpu.opcode_state.operand =
                    Operand::Address(u16::from_le_bytes([cpu.opcode_state.latch, adh]));
            }
            _ => run_operation(cpu, bus),
        },
        Addressing::ZPage => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.operand = Operand::Address(cpu.read_byte(bus, cpu.pc) as u16);
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            2 => {
                run_operation(cpu, bus);
            }
            _=> {}
        },
        Addressing::IndexedZPageX => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            2 => {
                cpu.read_byte(bus, cpu.opcode_state.latch as u16);
                cpu.opcode_state.operand =
                    Operand::Address(cpu.opcode_state.latch.wrapping_add(cpu.x) as u16);
            }
            _ => run_operation(cpu, bus),
        },
        Addressing::IndexedZPageY => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            2 => {
                cpu.read_byte(bus, cpu.opcode_state.latch as u16);
                cpu.opcode_state.operand =
                    Operand::Address(cpu.opcode_state.latch.wrapping_add(cpu.y) as u16);
            }
            _ => run_operation(cpu, bus),
        },
        Addressing::IndexedAbsoluteX => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            2 => {
                let adh = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);

                cpu.opcode_state.operand =
                    Operand::Address(u16::from_le_bytes([cpu.opcode_state.latch, adh]));
            }
            3 => {
                let address = match cpu.opcode_state.operand {
                    Address(address) => address,
                    _ => panic!("expected address operand"),
                };

                cpu.opcode_state.operand = Operand::Address(address.wrapping_add(cpu.x as u16));

                if matches!(cpu.opcode_state.current_opcode.operation, STA | STX | STY) {
                    let no_carry = (address & 0xFF00) | (address as u8).wrapping_add(cpu.x) as u16;
                    cpu.read_byte(bus, no_carry);
                } else {
                    if (address as u8).checked_add(cpu.x).is_none() {
                        cpu.opcode_state.page_cross = true;

                        cpu.read_byte(
                            bus,
                            (address & 0xFF00) | (address as u8).wrapping_add(cpu.x) as u16,
                        );
                    } else {
                        run_operation(cpu, bus);
                    }
                }
            }
            4 => {
                if cpu.opcode_state.page_cross
                    || matches!(cpu.opcode_state.current_opcode.operation, STA | STX | STY)
                {
                    run_operation(cpu, bus);
                }
            }
            _ => {}
        },
        Addressing::IndexedAbsoluteY => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            2 => {
                let adh = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);

                cpu.opcode_state.operand =
                    Operand::Address(u16::from_le_bytes([cpu.opcode_state.latch, adh]));
            }
            3 => {
                let address = match cpu.opcode_state.operand {
                    Address(address) => address,
                    _ => panic!("expected address operand"),
                };

                cpu.opcode_state.operand = Operand::Address(address.wrapping_add(cpu.y as u16));

                if matches!(cpu.opcode_state.current_opcode.operation, STA | STX | STY) {
                    let no_carry = (address & 0xFF00) | (address as u8).wrapping_add(cpu.y) as u16;
                    cpu.read_byte(bus, no_carry);
                } else {
                    if (address as u8).checked_add(cpu.y).is_none() {
                        cpu.opcode_state.page_cross = true;

                        cpu.read_byte(
                            bus,
                            (address & 0xFF00) | (address as u8).wrapping_add(cpu.y) as u16,
                        );
                    } else {
                        run_operation(cpu, bus);
                    }
                }
            }
            4 => {
                if cpu.opcode_state.page_cross
                    || matches!(cpu.opcode_state.current_opcode.operation, STA | STX | STY)
                {
                    run_operation(cpu, bus);
                }
            }
            _ => {}
        },
        Addressing::Implied => {
            cpu.opcode_state.operand = Operand::Implied;
            run_operation(cpu, bus);
        }
        Addressing::Relative => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);

                let final_addr = cpu
                    .pc
                    .wrapping_add_signed((cpu.opcode_state.latch as i8) as i16);

                cpu.opcode_state.page_cross = (cpu.pc & 0xFF00) != (final_addr & 0xFF00);
                cpu.opcode_state.operand = Address(final_addr);
                run_operation(cpu, bus);
            }
            2 => {
                if cpu.opcode_state.branch_taken {
                    let seq_pc = cpu
                        .pc
                        .wrapping_sub_signed((cpu.opcode_state.latch as i8) as i16);
                    let no_carry = (seq_pc & 0xFF00)
                        | ((seq_pc as u8).wrapping_add_signed(cpu.opcode_state.latch as i8) as u16);

                    cpu.read_byte(bus, no_carry);
                }
            }
            3 => {
                if cpu.opcode_state.branch_taken && cpu.opcode_state.page_cross {
                    cpu.read_byte(bus, cpu.pc);
                }
            }
            _ => {}
        },
        Addressing::IndexedIndirect => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);
            }

            2 => _ = cpu.read_byte(bus, cpu.opcode_state.latch as u16),

            3 => {
                cpu.opcode_state.operand = Address(
                    cpu.read_byte(bus, (cpu.opcode_state.latch).wrapping_add(cpu.x) as u16) as u16,
                )
            }

            4 => {
                let adl = match cpu.opcode_state.operand {
                    Address(address) => address as u8,
                    _ => panic!("expected address operand"),
                };
                let adh = cpu.read_byte(
                    bus,
                    cpu.opcode_state.latch.wrapping_add(cpu.x).wrapping_add(1) as u16,
                );

                cpu.opcode_state.operand = Address(u16::from_le_bytes([adl, adh]));
                run_operation(cpu, bus);
            }
            _ => {}
        },
        Addressing::IndirectIndexed => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            2 => {
                cpu.opcode_state.operand =
                    Address(cpu.read_byte(bus, cpu.opcode_state.latch as u16) as u16)
            }
            3 => match cpu.opcode_state.operand {
                Address(address) => {
                    let bal = address as u8;
                    let bah = cpu.read_byte(bus, cpu.opcode_state.latch.wrapping_add(1) as u16);
                    cpu.opcode_state.operand = Address(u16::from_le_bytes([bal, bah]))
                }
                _ => panic!("expected address operand"),
            },
            4 => {
                let address = match cpu.opcode_state.operand {
                    Address(address) => address,
                    _ => panic!("expected address operand"),
                };
                cpu.opcode_state.operand = Address(address.wrapping_add(cpu.y as u16));
                if matches!(cpu.opcode_state.current_opcode.operation, STA | STX | STY) {
                    let no_carry =
                        (address & 0xFF00) | ((address & 0xFF) as u8).wrapping_add(cpu.y) as u16;

                    cpu.read_byte(bus, no_carry);
                } else {
                    let no_carry =
                        (address & 0xFF00) | ((address as u8).wrapping_add(cpu.y) as u16);

                    if (address as u8).checked_add(cpu.y).is_none() {
                        cpu.opcode_state.page_cross = true;
                        cpu.read_byte(bus, no_carry);
                    } else {
                        run_operation(cpu, bus);
                    }
                }
            }
            5 => {
                if cpu.opcode_state.page_cross
                    || matches!(cpu.opcode_state.current_opcode.operation, STA | STX | STY)
                {
                    run_operation(cpu, bus);
                }
            }

            _ => {}
        },
        Addressing::AbsoluteIndirect => match cpu.opcode_state.opcode_cycle {
            1 => {
                cpu.opcode_state.latch = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            2 => {
                let ial = cpu.opcode_state.latch;

                let iah = cpu.read_byte(bus, cpu.pc);
                cpu.pc = cpu.pc.wrapping_add(1);

                cpu.opcode_state.operand = Address(u16::from_le_bytes([ial, iah]));
            }
            3 => {
                let address = match cpu.opcode_state.operand {
                    Address(address) => address,
                    _ => panic!("expected address operand"),
                };

                cpu.opcode_state.latch = cpu.read_byte(bus, address);
            }
            4 => {
                let address = match cpu.opcode_state.operand {
                    Address(address) => address,
                    _ => panic!("expected address operand"),
                };

                let adl = cpu.opcode_state.latch;

                let adh = if (address as u8) == 0xFF {
                    cpu.read_byte(bus, u16::from_le_bytes([0x00, (address >> 8) as u8]))
                } else {
                    cpu.read_byte(bus, address.wrapping_add(1))
                };

                cpu.opcode_state.operand = Address(u16::from_le_bytes([adl, adh]));
                run_operation(cpu, bus);
            }
            _ => {}
        },
    }
}
