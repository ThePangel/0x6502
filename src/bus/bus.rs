pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, byte: u8);

    fn console_write(&self, byte: u8);
    fn console_read(&self) -> Option<u8>;
}
