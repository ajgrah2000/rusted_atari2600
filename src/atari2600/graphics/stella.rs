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

pub struct Stella {
    screen_start_clock:u64,
}

impl Stella {
    pub const FRAME_WIDTH:u16 = 160;
    pub const HORIZONTAL_BLANK:u16 = 68;
    pub const HORIZONTAL_TICKS:u16 = Stella::FRAME_WIDTH + Stella::HORIZONTAL_BLANK;

    pub fn new() -> Self {
        Self { screen_start_clock: 0,
        }
    }

    pub fn write(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        match address & 0x3F {

            0x02 => { self.write_wsync(clock, address, data); }
            0x03 => { self.write_rsync(clock, address, data); }
            _ => { 
                // Not implemented yet
            }
        }
        // TODO
    }
    pub fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        // TODO
        println!("Stella read: {:X}", address);
        0
    }

    pub fn write_wsync(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) 
    {
        if (clock.ticks - self.screen_start_clock) % Stella::HORIZONTAL_TICKS as u64 > 3 {
            clock.ticks += Stella::HORIZONTAL_TICKS as u64 - (clock.ticks - self.screen_start_clock) % Stella::HORIZONTAL_TICKS as u64;
        }
    }

    pub fn write_rsync(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) 
    {
        let FUDGE = 3;
        if (clock.ticks - self.screen_start_clock) > 3 {
            clock.ticks += Stella::HORIZONTAL_TICKS as u64 - (clock.ticks - self.screen_start_clock + FUDGE) % Stella::HORIZONTAL_TICKS as u64
        }
    }
}

impl io::ReadWriteMemory for Stella{
    fn write(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.write(clock, address, data);
    }
    fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        self.read(clock, address)
    }
 }
