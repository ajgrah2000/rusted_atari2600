use super::super::io;

pub struct Riot {}

impl Riot {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&mut self, address: u16, data: u8) {
        // TODO
    }
    pub fn read(&mut self, address: u16) -> u8 {
        // TODO
        0
    }
}

impl io::ReadWriteMemory for Riot {
    fn write(&mut self, address: u16, data: u8) {
        self.write(address, data);
    }
    fn read(&mut self, address: u16) -> u8 {
        self.read(address)
    }
}
