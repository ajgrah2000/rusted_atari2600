use super::super::io;
use super::super::clocks;

pub struct Constants {}

impl Constants {
    pub const ATARI2600_WIDTH:  u16 = 160;
    pub const ATARI2600_HEIGHT: u16 = 280;

    pub const PIXEL_WIDTH:  u8 = 4;
    pub const PIXEL_HEIGHT: u8 = 2;

    pub const BLIT_WIDTH:  u16 = Constants::ATARI2600_WIDTH  * (Constants::PIXEL_WIDTH  as u16);
    pub const BLIT_HEIGHT: u16 = Constants::ATARI2600_HEIGHT * (Constants::PIXEL_HEIGHT as u16);
}

pub struct Stella {}

impl Stella {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&mut self, clock: &clocks::Clock, address: u16, data: u8) {
        // TODO
    }
    pub fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        // TODO
        0
    }
}

impl io::ReadWriteMemory for Stella{
    fn write(&mut self, clock: &clocks::Clock, address: u16, data: u8) {
        self.write(clock, address, data);
    }
    fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        self.read(clock, address)
    }
 }
