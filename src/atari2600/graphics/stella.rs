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

    pub const VSYNC_MASK:u8 = 0x2;
    pub const VSYNC_ON:u8   = 0x2;
    pub const VSYNC_OFF:u8  = 0x0;

    pub const DEFAULT_COLOUR:display::Colour = display::Colour::new(0, 0, 0);
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
  // Line state used per stella line.
    p_colour: (display::Colour, display::Colour),
    background_colour: display::Colour,
    playfield_colour: display::Colour,
    ctrlpf:u8,
    hmp:(u8, u8),
    hmm:(u8, u8),
    hmbl:u8,
}

impl LineState {
    pub fn new() -> Self {
        Self {
            p_colour: (Constants::DEFAULT_COLOUR, Constants::DEFAULT_COLOUR),
            background_colour: Constants::DEFAULT_COLOUR,
            playfield_colour: Constants::DEFAULT_COLOUR,
            ctrlpf: 0,
            hmp: (0,0),
            hmm: (0,0),
            hmbl: 0,
        }
    }
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

    pub fn get_color(&self, colour:u8) -> display::Colour {
        self.colours[colour as usize >> 1]
    }
}

pub struct Stella {
    screen_start_clock:clocks::ClockType,
    paddle_start_clock:clocks::ClockType,
    last_screen_update_clock:clocks::ClockType,
    next_line:LineState,
    is_vsync:bool,
    is_blank:bool,
    is_input_latched:bool,
    is_update_time:bool,
}

impl Stella {
    pub const FRAME_WIDTH:u16 = 160;
    pub const HORIZONTAL_BLANK:u16 = 68;
    pub const HORIZONTAL_TICKS:clocks::ClockType = (Stella::FRAME_WIDTH + Stella::HORIZONTAL_BLANK) as clocks::ClockType;
    pub const INPUT_45_LATCH_MASK:u8 = 0x40;
    pub const BLANK_PADDLE_RECHARGE:u8 = 0x80;
    pub const BLANK_MASK:u8 = 0x2;
    pub const BLANK_ON:u8 = 0x2;
    pub const BLANK_OFF:u8 = 0x0;

    pub fn new() -> Self {
        let mut colours = Colours::new();
        colours.load("palette.dat");

        Self { 
            screen_start_clock: 0,
            paddle_start_clock: 0,
            last_screen_update_clock: 0,
            next_line: LineState::new(),
            is_vsync:false,
            is_blank:false,
            is_input_latched:false,
            is_update_time:false,
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

            0x00 => {self.write_vsync(clock, address, data); }
            0x01 => {self.write_vblank(clock, address, data); }
            0x02 => {self.write_wsync(clock, address, data); }
            0x03 => {self.write_rsync(clock, address, data); }
            0x04 => {self.write_nusiz0(clock, address, data); }
            0x05 => {self.write_nusiz1(clock, address, data); }
            0x06 => {self.write_colump0(clock, address, data); }
            0x07 => {self.write_colump1(clock, address, data); }
            0x08 => {self.write_colupf(clock, address, data); }
            0x09 => {self.write_colubk(clock, address, data); }
            0x0A => {self.write_ctrlpf(clock, address, data); }
            0x0B => {self.write_refp0(clock, address, data); }
            0x0C => {self.write_refp1(clock, address, data); }
            0x0D => {self.write_pf0(clock, address, data); }
            0x0E => {self.write_pf1(clock, address, data); }
            0x0F => {self.write_pf2(clock, address, data); }
            0x10 => {self.write_resp0(clock, address, data); }
            0x11 => {self.write_resp1(clock, address, data); }
            0x12 => {self.write_resm0(clock, address, data); }
            0x13 => {self.write_resm1(clock, address, data); }
            0x14 => {self.write_resbl(clock, address, data); }
// TODO: Add 'tiasound'            
//            0x15 => {self.tiasound.write_audio_ctrl_0(clock, address, data); }
//            0x16 => {self.tiasound.write_audio_ctrl_1(clock, address, data); }
//            0x17 => {self.tiasound.write_audio_freq_0(clock, address, data); }
//            0x18 => {self.tiasound.write_audio_freq_1(clock, address, data); }
//            0x19 => {self.tiasound.write_audio_vol_0(clock, address, data); }
//            0x1A => {self.tiasound.write_audio_vol_1(clock, address, data); }
            0x1B => {self.write_grp0(clock, address, data); }
            0x1C => {self.write_grp1(clock, address, data); }
            0x1D => {self.write_enam0(clock, address, data); }
            0x1E => {self.write_enam1(clock, address, data); }
            0x1F => {self.write_enabl(clock, address, data); }
            0x20 => {self.write_hmp0(clock, address, data); }
            0x21 => {self.write_hmp1(clock, address, data); }
            0x22 => {self.write_hmm0(clock, address, data); }
            0x23 => {self.write_hmm1(clock, address, data); }
            0x24 => {self.write_hmbl(clock, address, data); }
            0x2A => {self.write_hmove(clock, address, data); }
            0x2B => {self.write_hclr(clock, address, data); }
            0x25 => {self.write_vdelp0(clock, address, data); }
            0x26 => {self.write_vdelp1(clock, address, data); }
            0x27 => {self.write_vdelbl(clock, address, data); }
            0x2C => {self.write_cxclr(clock, address, data); }
            _ => { 
                println!("Stella write not supported {}", address & 0x3F);
            }
        }
    }

    fn write_vsync(&mut self, clock: &mut clocks::Clock, address: u16, data: u8)  {
        if false == self.is_vsync {
            if Constants::VSYNC_ON == (data & Constants::VSYNC_MASK) {
                self.is_update_time = true;
                self.is_vsync = true;
            }
        } else {
            if Constants::VSYNC_OFF == (data & Constants::VSYNC_MASK) {
                self.is_vsync = false;
                self.screen_start_clock = clock.ticks.wrapping_sub(Stella::HORIZONTAL_TICKS).wrapping_add((Stella::HORIZONTAL_TICKS.wrapping_sub(clock.ticks).wrapping_add(self.screen_start_clock)) % Stella::HORIZONTAL_TICKS);
                self.last_screen_update_clock = self.screen_start_clock;
            }
        }
    }

    fn write_vblank(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        if 0 != data & Stella::INPUT_45_LATCH_MASK {
            self.is_input_latched = true;
        } else {
            self.is_input_latched = false;
        }

        if (data & Stella::BLANK_PADDLE_RECHARGE) == Stella::BLANK_PADDLE_RECHARGE {
            self.paddle_start_clock = clock.ticks;
        }

        if (data & Stella::BLANK_MASK) == Stella::BLANK_ON {
            self.is_blank = true;
        } else if (data & Stella::BLANK_MASK) == Stella::BLANK_OFF {
            self.is_blank = false;
        }
    }

    fn write_wsync(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        if (clock.ticks - self.screen_start_clock) % Stella::HORIZONTAL_TICKS > 3 {
            clock.ticks += Stella::HORIZONTAL_TICKS - (clock.ticks - self.screen_start_clock) % Stella::HORIZONTAL_TICKS;
        }
    }

    fn write_rsync(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        let fudge = 3;

        if (clock.ticks - self.screen_start_clock) > 3 {
            clock.ticks += Stella::HORIZONTAL_TICKS - (clock.ticks - self.screen_start_clock + fudge) % Stella::HORIZONTAL_TICKS; 
        }
    }

    fn write_nusiz0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_nusiz1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_colump0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_colump1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_colupf(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_colubk(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_ctrlpf(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_refp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_refp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_pf0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_pf1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_pf2(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_resp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_resp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_resm0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_resm1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_resbl(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_grp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_grp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_enam0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_enam1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_enabl(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_hmp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_hmp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_hmm0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_hmm1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_hmbl(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_hmove(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_hclr(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_vdelp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_vdelp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_vdelbl(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_cxclr(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
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
