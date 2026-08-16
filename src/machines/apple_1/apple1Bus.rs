use std::mem;

use crate::bus::Bus;

const WOZMON: &[u8] = include_bytes!("./roms/wozmon.bin");
const WOZACI: &[u8] = include_bytes!("./roms/wozaci.bin");
const INTBASIC: &[u8] = include_bytes!("./roms/apple1basic.bin");

struct Apple1Bus {
    pub memory: [u8; 65536],
}

impl Apple1Bus {
    fn new() -> Self {
        let mut memory: [u8; 65536] = [0; 65536];

        memory[0xFF00..=0xFFFF].copy_from_slice(WOZMON);
        memory[0xC100..=0xC1FF].copy_from_slice(WOZACI);
        memory[0xE000..=0xEFFF].copy_from_slice(INTBASIC);

        Apple1Bus { memory }
    }
}

impl Bus for Apple1Bus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // RAM (8K system per wikipedia)
            0x0000..=0x0FFF | 0xE000..=0xEFFF => self.memory[addr as usize],
            // ACI ROM
            0xC100..=0xC1FF => self.memory[addr as usize],
            // Peripheral Interface Adapter (KB and Display)
            0xD010..=0xD013 => {
                if addr == 0xD010 {
                    self.memory[0xD011] = 0;
                };

                self.memory[addr as usize]
            }
            // Wozmon PROM
            0xFF00..=0xFFFF => self.memory[addr as usize],
            _ => todo!("temp wildcard"),
        }
    }
    fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            //RAM (8K system per wikipedia)
            0x0000..=0x0FFF | 0xE000..=0xEFFF => self.memory[addr as usize] = byte,
            // ACI write
            0xC028 => todo!("Maybe implement cassette write idk"),
            // Peripheral Interface Adapter (KB and Display)
            0xD010..=0xD013 => {
                if addr == 0xD012 {
                    self.console_write(byte);
                    self.memory[addr as usize] = byte | 0x80
                } else if addr == 0xD010 {
                    self.memory[addr as usize] = byte | 0x80
                }
            }
            _ => todo!("temp wildcard"),
        }
    }
    fn console_read(&self) -> Option<u8> {
        todo!()
    }
    fn console_write(&self, byte: u8) {
        todo!()
    }
}
