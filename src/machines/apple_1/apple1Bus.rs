use crate::bus::Bus;

struct Apple1Bus {
    pub memory: [u8; 0x10000],
}

impl Apple1Bus {
    fn new() -> Self {
        todo!()
    }
}

impl Bus for Apple1Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // RAM (8K system per wikipedia)
            0x0000..=0x0FFF | 0xE000..=0xEFFF => self.memory[addr as usize],
            // ACI ROM
            0xC100..=0xC1FF => todo!("Maybe implement cassette rom idk"),
            // Peripheral Interface Adapter (KB and Display)
            0xD010..=0xD013 => todo!("Handle IO"),
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
            0xD010..=0xD013 => todo!("Handle IO"),
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
