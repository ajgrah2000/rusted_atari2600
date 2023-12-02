use super::super::io;
use super::super::clocks;
use super::display;
use std;
use std::io::BufRead;

pub struct Constants {}

impl Constants {
    pub const ATARI2600_WIDTH:  u16 = 160;
    pub const ATARI2600_HEIGHT: u16 = 280;

    pub const PIXEL_WIDTH:  u8 = 4;
    pub const PIXEL_HEIGHT: u8 = 2;

    pub const BLIT_WIDTH:  u16 = Constants::ATARI2600_WIDTH  * (Constants::PIXEL_WIDTH  as u16);
    pub const BLIT_HEIGHT: u16 = Constants::ATARI2600_HEIGHT * (Constants::PIXEL_HEIGHT as u16);
}

pub struct PlayfieldState {
}

pub struct BallState {
}

pub struct MissileState {
}

pub struct PlayerState {
}

pub struct LineState {
}

pub struct CollisionState {
}

pub struct Colours {
    colours: Vec<display::Colour>, 
}

impl Colours {
    pub const NUM_COLOURS:u8 = 128;

    pub fn new() -> Self {
        Self {
            colours: vec![display::Colour::new(0, 0, 0); Colours::NUM_COLOURS as usize],
        }
    }

    pub fn load(&mut self, palette_filename: &str) {
        let buf_read = std::io::BufReader::new(std::fs::File::open(palette_filename).expect(format!("file not found! {}", palette_filename).as_str()));
        let lines:Vec<String> = buf_read.lines().map(|x| x.unwrap()).collect();
        for (i, line) in lines.iter().enumerate() {
            let line_without_comments = &line[0..line.find("#").unwrap_or(line.len())].trim_end_matches(' ');
            let values:Vec<u8> = line_without_comments.split(' ').collect::<Vec<&str>>().iter().map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
            self.colours[i] = display::Colour::new(values[0], values[1], values[2]);
        }
    }
}

pub struct Stella {
    screen_start_clock:u64,
}

impl Stella {
    pub const FRAME_WIDTH:u16 = 160;
    pub const HORIZONTAL_BLANK:u16 = 68;
    pub const HORIZONTAL_TICKS:u16 = Stella::FRAME_WIDTH + Stella::HORIZONTAL_BLANK;

    pub fn new() -> Self {
        let mut colours = Colours::new();
        colours.load("palette.dat");

        Self { screen_start_clock: 0,
        }
    }

    pub fn write(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO

        self. write_functions(clock, address, data);
    }
    pub fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        // TODO
        println!("Stella read: {:X}", address);
        0
    }

    fn write_functions(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
        match address & 0x3F {

            0x02 => { self.write_wsync(clock, address, data); }
            0x03 => { self.write_rsync(clock, address, data); }
            _ => { 
                // Not implemented yet
            }
        }
    }

    pub fn write_wsync(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) 
    {
        if (clock.ticks - self.screen_start_clock) % Stella::HORIZONTAL_TICKS as u64 > 3 {
            clock.ticks += Stella::HORIZONTAL_TICKS as u64 - (clock.ticks - self.screen_start_clock) % Stella::HORIZONTAL_TICKS as u64;
        }
    }

    pub fn write_rsync(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) 
    {
        let fudge = 3;

        if (clock.ticks - self.screen_start_clock) > 3 {
            clock.ticks += Stella::HORIZONTAL_TICKS as u64 - (clock.ticks - self.screen_start_clock + fudge) % Stella::HORIZONTAL_TICKS as u64
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
