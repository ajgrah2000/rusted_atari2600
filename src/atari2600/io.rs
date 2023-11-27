use super::clocks;

pub trait ReadWriteMemory {
    fn read(&mut self, clock: &clocks::Clock, address:u16) -> u8;
    fn write(&mut self, clock: &clocks::Clock, address:u16, data:u8);
}
