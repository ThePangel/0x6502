use crate::cpu::cpu6502::Cpu6502;

pub fn set_carry(cpu: &mut Cpu6502, status: bool) {
    if status {
        cpu.p |= 0x01
    } else {
        cpu.p &= !0x01
    }
}

pub fn set_zero(cpu: &mut Cpu6502, status: bool) {
    if status {
        cpu.p |= 0x02
    } else {
        cpu.p &= !0x02
    } 
}

pub fn set_interrupt_disable(cpu: &mut Cpu6502, status: bool) {
    if status {
        cpu.p |= 0x04
    } else {
        cpu.p &= !0x04
    }
}

pub fn set_decimal(cpu: &mut Cpu6502, status: bool) {
    if status {
        cpu.p |= 0x08
    } else {
        cpu.p &= !0x08
    }
}

pub fn set_break(cpu: &mut Cpu6502, status: bool) {
    if status {
        cpu.p |= 0x10
    } else {
        cpu.p &= !0x10
    }
}

pub fn set_overflow(cpu: &mut Cpu6502, status: bool) {
    if status {
        cpu.p |= 0x40
    } else {
        cpu.p &= !0x40
    }
}

pub fn set_negative(cpu: &mut Cpu6502, status: bool) {
    if status {
        cpu.p |= 0x80
    } else {
        cpu.p &= !0x80
    }
}

pub fn get_carry(cpu: &Cpu6502) -> bool {
    (cpu.p & 0x01) != 0
}

pub fn get_zero(cpu: &Cpu6502) -> bool {
    (cpu.p & 0x02) != 0
}

pub fn get_interrupt_disable(cpu: &Cpu6502) -> bool {
    (cpu.p & 0x04) != 0
}

pub fn get_decimal(cpu: &Cpu6502) -> bool {
    (cpu.p & 0x08) != 0
}

pub fn get_break(cpu: &Cpu6502) -> bool {
    (cpu.p & 0x10) != 0
}

pub fn get_overflow(cpu: &Cpu6502) -> bool {
    (cpu.p & 0x40) != 0
}

pub fn get_negative(cpu: &Cpu6502) -> bool {
    (cpu.p & 0x80) != 0
}
