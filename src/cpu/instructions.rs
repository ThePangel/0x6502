pub enum Operation {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

pub enum Addressing {
    Accumulator,
    Immediate,
    Absolute,
    ZPage,
    IndexedZPageX,
    IndexedZPageY,
    IndexedAbsoluteX,
    IndexedAbsoluteY,
    Implied,
    Relative,
    IndexedIndirect,
    IndirectIndexed,
    AbsoluteIndirect,
}

pub struct Instruction {
    pub operation: Operation,
    pub addressing: Addressing,
}

pub enum Operand {
    Accumulator,
    Address(u16),
    Implied,
}

pub fn get_instruction(opcode: u8) -> Instruction {
    match opcode {
        0x69 => Instruction {
            operation: Operation::ADC,
            addressing: Addressing::Immediate,
        },
        0x6D => Instruction {
            operation: Operation::ADC,
            addressing: Addressing::Absolute,
        },
        0x65 => Instruction {
            operation: Operation::ADC,
            addressing: Addressing::ZPage,
        },
        0x61 => Instruction {
            operation: Operation::ADC,
            addressing: Addressing::IndexedIndirect,
        },
        0x71 => Instruction {
            operation: Operation::ADC,
            addressing: Addressing::IndirectIndexed,
        },
        0x75 => Instruction {
            operation: Operation::ADC,
            addressing: Addressing::IndexedZPageX,
        },
        0x7D => Instruction {
            operation: Operation::ADC,
            addressing: Addressing::IndexedAbsoluteX,
        },
        0x79 => Instruction {
            operation: Operation::ADC,
            addressing: Addressing::IndexedAbsoluteY,
        },

        0x29 => Instruction {
            operation: Operation::AND,
            addressing: Addressing::Immediate,
        },
        0x2D => Instruction {
            operation: Operation::AND,
            addressing: Addressing::Absolute,
        },
        0x25 => Instruction {
            operation: Operation::AND,
            addressing: Addressing::ZPage,
        },
        0x21 => Instruction {
            operation: Operation::AND,
            addressing: Addressing::IndexedIndirect,
        },
        0x31 => Instruction {
            operation: Operation::AND,
            addressing: Addressing::IndirectIndexed,
        },
        0x35 => Instruction {
            operation: Operation::AND,
            addressing: Addressing::IndexedZPageX,
        },
        0x3D => Instruction {
            operation: Operation::AND,
            addressing: Addressing::IndexedAbsoluteX,
        },
        0x39 => Instruction {
            operation: Operation::AND,
            addressing: Addressing::IndexedAbsoluteY,
        },

        0x0E => Instruction {
            operation: Operation::ASL,
            addressing: Addressing::Absolute,
        },
        0x06 => Instruction {
            operation: Operation::ASL,
            addressing: Addressing::ZPage,
        },
        0x0A => Instruction {
            operation: Operation::ASL,
            addressing: Addressing::Accumulator,
        },
        0x16 => Instruction {
            operation: Operation::ASL,
            addressing: Addressing::IndexedZPageX,
        },
        0x1E => Instruction {
            operation: Operation::ASL,
            addressing: Addressing::IndexedAbsoluteX,
        },

        0x90 => Instruction {
            operation: Operation::BCC,
            addressing: Addressing::Relative,
        },

        0xB0 => Instruction {
            operation: Operation::BCS,
            addressing: Addressing::Relative,
        },

        0xF0 => Instruction {
            operation: Operation::BEQ,
            addressing: Addressing::Relative,
        },

        0x2C => Instruction {
            operation: Operation::BIT,
            addressing: Addressing::Absolute,
        },
        0x24 => Instruction {
            operation: Operation::BIT,
            addressing: Addressing::ZPage,
        },

        0x30 => Instruction {
            operation: Operation::BMI,
            addressing: Addressing::Relative,
        },

        0xD0 => Instruction {
            operation: Operation::BNE,
            addressing: Addressing::Relative,
        },

        0x10 => Instruction {
            operation: Operation::BPL,
            addressing: Addressing::Relative,
        },

        0x00 => Instruction {
            operation: Operation::BRK,
            addressing: Addressing::Implied,
        },

        0x50 => Instruction {
            operation: Operation::BVC,
            addressing: Addressing::Relative,
        },

        0x70 => Instruction {
            operation: Operation::BVS,
            addressing: Addressing::Relative,
        },

        0x18 => Instruction {
            operation: Operation::CLC,
            addressing: Addressing::Implied,
        },

        0xD8 => Instruction {
            operation: Operation::CLD,
            addressing: Addressing::Implied,
        },

        0x58 => Instruction {
            operation: Operation::CLI,
            addressing: Addressing::Implied,
        },

        0xB8 => Instruction {
            operation: Operation::CLV,
            addressing: Addressing::Implied,
        },

        0xC9 => Instruction {
            operation: Operation::CMP,
            addressing: Addressing::Immediate,
        },
        0xCD => Instruction {
            operation: Operation::CMP,
            addressing: Addressing::Absolute,
        },
        0xC5 => Instruction {
            operation: Operation::CMP,
            addressing: Addressing::ZPage,
        },
        0xC1 => Instruction {
            operation: Operation::CMP,
            addressing: Addressing::IndexedIndirect,
        },
        0xD1 => Instruction {
            operation: Operation::CMP,
            addressing: Addressing::IndirectIndexed,
        },
        0xD5 => Instruction {
            operation: Operation::CMP,
            addressing: Addressing::IndexedZPageX,
        },
        0xDD => Instruction {
            operation: Operation::CMP,
            addressing: Addressing::IndexedAbsoluteX,
        },
        0xD9 => Instruction {
            operation: Operation::CMP,
            addressing: Addressing::IndexedAbsoluteY,
        },

        0xE0 => Instruction {
            operation: Operation::CPX,
            addressing: Addressing::Immediate,
        },
        0xEC => Instruction {
            operation: Operation::CPX,
            addressing: Addressing::Absolute,
        },
        0xE4 => Instruction {
            operation: Operation::CPX,
            addressing: Addressing::ZPage,
        },

        0xC0 => Instruction {
            operation: Operation::CPY,
            addressing: Addressing::Immediate,
        },
        0xCC => Instruction {
            operation: Operation::CPY,
            addressing: Addressing::Absolute,
        },
        0xC4 => Instruction {
            operation: Operation::CPY,
            addressing: Addressing::ZPage,
        },

        0xCE => Instruction {
            operation: Operation::DEC,
            addressing: Addressing::Absolute,
        },
        0xC6 => Instruction {
            operation: Operation::DEC,
            addressing: Addressing::ZPage,
        },
        0xD6 => Instruction {
            operation: Operation::DEC,
            addressing: Addressing::IndexedZPageX,
        },
        0xDE => Instruction {
            operation: Operation::DEC,
            addressing: Addressing::IndexedAbsoluteX,
        },

        0xCA => Instruction {
            operation: Operation::DEX,
            addressing: Addressing::Implied,
        },

        0x88 => Instruction {
            operation: Operation::DEY,
            addressing: Addressing::Implied,
        },

        0x49 => Instruction {
            operation: Operation::EOR,
            addressing: Addressing::Immediate,
        },
        0x4D => Instruction {
            operation: Operation::EOR,
            addressing: Addressing::Absolute,
        },
        0x45 => Instruction {
            operation: Operation::EOR,
            addressing: Addressing::ZPage,
        },
        0x41 => Instruction {
            operation: Operation::EOR,
            addressing: Addressing::IndexedIndirect,
        },
        0x51 => Instruction {
            operation: Operation::EOR,
            addressing: Addressing::IndirectIndexed,
        },
        0x55 => Instruction {
            operation: Operation::EOR,
            addressing: Addressing::IndexedZPageX,
        },
        0x5D => Instruction {
            operation: Operation::EOR,
            addressing: Addressing::IndexedAbsoluteX,
        },
        0x59 => Instruction {
            operation: Operation::EOR,
            addressing: Addressing::IndexedAbsoluteY,
        },

        0xEE => Instruction {
            operation: Operation::INC,
            addressing: Addressing::Absolute,
        },
        0xE6 => Instruction {
            operation: Operation::INC,
            addressing: Addressing::ZPage,
        },
        0xF6 => Instruction {
            operation: Operation::INC,
            addressing: Addressing::IndexedZPageX,
        },
        0xFE => Instruction {
            operation: Operation::INC,
            addressing: Addressing::IndexedAbsoluteX,
        },

        0xE8 => Instruction {
            operation: Operation::INX,
            addressing: Addressing::Implied,
        },

        0xC8 => Instruction {
            operation: Operation::INY,
            addressing: Addressing::Implied,
        },

        0x4C => Instruction {
            operation: Operation::JMP,
            addressing: Addressing::Absolute,
        },
        0x6C => Instruction {
            operation: Operation::JMP,
            addressing: Addressing::AbsoluteIndirect,
        },

        0x20 => Instruction {
            operation: Operation::JSR,
            addressing: Addressing::Absolute,
        },

        0xA9 => Instruction {
            operation: Operation::LDA,
            addressing: Addressing::Immediate,
        },
        0xAD => Instruction {
            operation: Operation::LDA,
            addressing: Addressing::Absolute,
        },
        0xA5 => Instruction {
            operation: Operation::LDA,
            addressing: Addressing::ZPage,
        },
        0xA1 => Instruction {
            operation: Operation::LDA,
            addressing: Addressing::IndexedIndirect,
        },
        0xB1 => Instruction {
            operation: Operation::LDA,
            addressing: Addressing::IndirectIndexed,
        },
        0xB5 => Instruction {
            operation: Operation::LDA,
            addressing: Addressing::IndexedZPageX,
        },
        0xBD => Instruction {
            operation: Operation::LDA,
            addressing: Addressing::IndexedAbsoluteX,
        },
        0xB9 => Instruction {
            operation: Operation::LDA,
            addressing: Addressing::IndexedAbsoluteY,
        },

        0xA2 => Instruction {
            operation: Operation::LDX,
            addressing: Addressing::Immediate,
        },
        0xAE => Instruction {
            operation: Operation::LDX,
            addressing: Addressing::Absolute,
        },
        0xA6 => Instruction {
            operation: Operation::LDX,
            addressing: Addressing::ZPage,
        },
        0xBE => Instruction {
            operation: Operation::LDX,
            addressing: Addressing::IndexedAbsoluteY,
        },
        0xB6 => Instruction {
            operation: Operation::LDX,
            addressing: Addressing::IndexedZPageY,
        },

        0xA0 => Instruction {
            operation: Operation::LDY,
            addressing: Addressing::Immediate,
        },
        0xAC => Instruction {
            operation: Operation::LDY,
            addressing: Addressing::Absolute,
        },
        0xA4 => Instruction {
            operation: Operation::LDY,
            addressing: Addressing::ZPage,
        },
        0xB4 => Instruction {
            operation: Operation::LDY,
            addressing: Addressing::IndexedZPageX,
        },
        0xBC => Instruction {
            operation: Operation::LDY,
            addressing: Addressing::IndexedAbsoluteX,
        },

        0x4E => Instruction {
            operation: Operation::LSR,
            addressing: Addressing::Absolute,
        },
        0x46 => Instruction {
            operation: Operation::LSR,
            addressing: Addressing::ZPage,
        },
        0x4A => Instruction {
            operation: Operation::LSR,
            addressing: Addressing::Accumulator,
        },
        0x56 => Instruction {
            operation: Operation::LSR,
            addressing: Addressing::IndexedZPageX,
        },
        0x5E => Instruction {
            operation: Operation::LSR,
            addressing: Addressing::IndexedAbsoluteX,
        },

        0xEA => Instruction {
            operation: Operation::NOP,
            addressing: Addressing::Implied,
        },

        0x09 => Instruction {
            operation: Operation::ORA,
            addressing: Addressing::Immediate,
        },
        0x0D => Instruction {
            operation: Operation::ORA,
            addressing: Addressing::Absolute,
        },
        0x05 => Instruction {
            operation: Operation::ORA,
            addressing: Addressing::ZPage,
        },
        0x01 => Instruction {
            operation: Operation::ORA,
            addressing: Addressing::IndexedIndirect,
        },
        0x11 => Instruction {
            operation: Operation::ORA,
            addressing: Addressing::IndirectIndexed,
        },
        0x15 => Instruction {
            operation: Operation::ORA,
            addressing: Addressing::IndexedZPageX,
        },
        0x1D => Instruction {
            operation: Operation::ORA,
            addressing: Addressing::IndexedAbsoluteX,
        },
        0x19 => Instruction {
            operation: Operation::ORA,
            addressing: Addressing::IndexedAbsoluteY,
        },

        0x48 => Instruction {
            operation: Operation::PHA,
            addressing: Addressing::Implied,
        },

        0x08 => Instruction {
            operation: Operation::PHP,
            addressing: Addressing::Implied,
        },

        0x68 => Instruction {
            operation: Operation::PLA,
            addressing: Addressing::Implied,
        },

        0x28 => Instruction {
            operation: Operation::PLP,
            addressing: Addressing::Implied,
        },

        0x2E => Instruction {
            operation: Operation::ROL,
            addressing: Addressing::Absolute,
        },
        0x26 => Instruction {
            operation: Operation::ROL,
            addressing: Addressing::ZPage,
        },
        0x2A => Instruction {
            operation: Operation::ROL,
            addressing: Addressing::Accumulator,
        },
        0x36 => Instruction {
            operation: Operation::ROL,
            addressing: Addressing::IndexedZPageX,
        },
        0x3E => Instruction {
            operation: Operation::ROL,
            addressing: Addressing::IndexedAbsoluteX,
        },

        0x6E => Instruction {
            operation: Operation::ROR,
            addressing: Addressing::Absolute,
        },
        0x66 => Instruction {
            operation: Operation::ROR,
            addressing: Addressing::ZPage,
        },
        0x6A => Instruction {
            operation: Operation::ROR,
            addressing: Addressing::Accumulator,
        },
        0x76 => Instruction {
            operation: Operation::ROR,
            addressing: Addressing::IndexedZPageX,
        },
        0x7E => Instruction {
            operation: Operation::ROR,
            addressing: Addressing::IndexedAbsoluteX,
        },

        0x40 => Instruction {
            operation: Operation::RTI,
            addressing: Addressing::Implied,
        },

        0x60 => Instruction {
            operation: Operation::RTS,
            addressing: Addressing::Implied,
        },

        0xE9 => Instruction {
            operation: Operation::SBC,
            addressing: Addressing::Immediate,
        },
        0xED => Instruction {
            operation: Operation::SBC,
            addressing: Addressing::Absolute,
        },
        0xE5 => Instruction {
            operation: Operation::SBC,
            addressing: Addressing::ZPage,
        },
        0xE1 => Instruction {
            operation: Operation::SBC,
            addressing: Addressing::IndexedIndirect,
        },
        0xF1 => Instruction {
            operation: Operation::SBC,
            addressing: Addressing::IndirectIndexed,
        },
        0xF5 => Instruction {
            operation: Operation::SBC,
            addressing: Addressing::IndexedZPageX,
        },
        0xFD => Instruction {
            operation: Operation::SBC,
            addressing: Addressing::IndexedAbsoluteX,
        },
        0xF9 => Instruction {
            operation: Operation::SBC,
            addressing: Addressing::IndexedAbsoluteY,
        },

        0x38 => Instruction {
            operation: Operation::SEC,
            addressing: Addressing::Implied,
        },

        0xF8 => Instruction {
            operation: Operation::SED,
            addressing: Addressing::Implied,
        },

        0x78 => Instruction {
            operation: Operation::SEI,
            addressing: Addressing::Implied,
        },

        0x8D => Instruction {
            operation: Operation::STA,
            addressing: Addressing::Absolute,
        },
        0x85 => Instruction {
            operation: Operation::STA,
            addressing: Addressing::ZPage,
        },
        0x81 => Instruction {
            operation: Operation::STA,
            addressing: Addressing::IndexedIndirect,
        },
        0x91 => Instruction {
            operation: Operation::STA,
            addressing: Addressing::IndirectIndexed,
        },
        0x95 => Instruction {
            operation: Operation::STA,
            addressing: Addressing::IndexedZPageX,
        },
        0x9D => Instruction {
            operation: Operation::STA,
            addressing: Addressing::IndexedAbsoluteX,
        },
        0x99 => Instruction {
            operation: Operation::STA,
            addressing: Addressing::IndexedAbsoluteY,
        },

        0x8E => Instruction {
            operation: Operation::STX,
            addressing: Addressing::Absolute,
        },
        0x86 => Instruction {
            operation: Operation::STX,
            addressing: Addressing::ZPage,
        },
        0x96 => Instruction {
            operation: Operation::STX,
            addressing: Addressing::IndexedZPageY,
        },

        0x8C => Instruction {
            operation: Operation::STY,
            addressing: Addressing::Absolute,
        },
        0x84 => Instruction {
            operation: Operation::STY,
            addressing: Addressing::ZPage,
        },
        0x94 => Instruction {
            operation: Operation::STY,
            addressing: Addressing::IndexedZPageX,
        },

        0xAA => Instruction {
            operation: Operation::TAX,
            addressing: Addressing::Implied,
        },

        0xA8 => Instruction {
            operation: Operation::TAY,
            addressing: Addressing::Implied,
        },

        0xBA => Instruction {
            operation: Operation::TSX,
            addressing: Addressing::Implied,
        },

        0x8A => Instruction {
            operation: Operation::TXA,
            addressing: Addressing::Implied,
        },

        0x9A => Instruction {
            operation: Operation::TXS,
            addressing: Addressing::Implied,
        },

        0x98 => Instruction {
            operation: Operation::TYA,
            addressing: Addressing::Implied,
        },

        _ => Instruction {
            operation: Operation::NOP,
            addressing: Addressing::Implied,
        },
    }
}
