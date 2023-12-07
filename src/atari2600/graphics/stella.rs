use super::super::io;
use super::super::clocks;
use super::display;
use std;
use std::io::BufRead;

pub struct Constants {}

impl Constants {
    pub const ATARI2600_WIDTH:  u16 = Stella::FRAME_WIDTH;
    pub const ATARI2600_HEIGHT: u16 = Stella::FRAME_HEIGHT;

    pub const PIXEL_WIDTH_STRETCH: u8 = 2;

    pub const VSYNC_MASK:u8 = 0x2;
    pub const VSYNC_ON:u8   = 0x2;
    pub const VSYNC_OFF:u8  = 0x0;

    pub const DEFAULT_COLOUR:display::Colour = display::Colour::new(0, 0, 0);
}

pub struct PlayfieldState {
    // Playfield state.
    // It's updated infrequently, so generate an entire scan each update and
    // return the lookup.

    pf0:u8,
    pf1:u8,
    pf2:u8,
    ctrlpf:u8,

    pf_lookup: Vec<bool>,
    pf0_lookup: Vec< Vec<bool> >,
    pf1_lookup: Vec< Vec<bool> >,
    pf2_lookup: Vec< Vec<bool> >,
}

impl PlayfieldState {
    pub const PLAYFIELD_LOOKUP_SIZE:usize = 256;
    pub const PLAYFIELD_LENGTH:usize = 8;
    pub const PLAYFIELD_EXPAND_SIZE:usize = 4;

    pub fn new() -> Self {
        let mut instance = Self {
            pf0: 0,
            pf1: 0,
            pf2: 0,
            ctrlpf: 0,
            pf_lookup: vec![false; Stella::FRAME_WIDTH as usize],
            pf0_lookup: vec![vec![false; 16]; 16],
            pf1_lookup: vec![vec![false; PlayfieldState::PLAYFIELD_LENGTH * PlayfieldState::PLAYFIELD_EXPAND_SIZE]; PlayfieldState::PLAYFIELD_LOOKUP_SIZE],
            pf2_lookup: vec![vec![false; PlayfieldState::PLAYFIELD_LENGTH * PlayfieldState::PLAYFIELD_EXPAND_SIZE]; PlayfieldState::PLAYFIELD_LOOKUP_SIZE],
        };

        instance.pre_calc_playfield();

        instance
    }

    fn pre_calc_playfield(&mut self) {
        // Pre-calc playfield lists. 
        //
        // Bit order for displaying pf1 is reverse to pf0 & pf2.
        // Order:
        // PF0: 4,5,6,7, PF1: 7,6,5,4,3,2,1,0 PF2: 0,1,2,3,4,5,6,7

        for i in 0..256 {
            let mut pf_lookup = vec![false;8];
            let mut mask = 1;
            for b in 0..8 {
                if 0 != i & mask {
                    pf_lookup[b] = true;
                }
                mask += mask;
            }

            // Expand to 4-pixels
            let mut pf_lookup_expanded = vec![false; 8 * PlayfieldState::PLAYFIELD_EXPAND_SIZE];
            for j in 0..pf_lookup.len() {
                for k in 0..PlayfieldState::PLAYFIELD_EXPAND_SIZE  {
                    pf_lookup_expanded[(j * PlayfieldState::PLAYFIELD_EXPAND_SIZE + k) as usize] = pf_lookup[j];
                }
            }

            self.pf2_lookup[i] = pf_lookup_expanded.clone();
            pf_lookup_expanded.reverse();
            self.pf1_lookup[i] = pf_lookup_expanded.clone();
        }

        // PF0 is only 4-bit encoding.
        for i in 0..16 {
            self.pf0_lookup[i] = self.pf2_lookup[i*16][16..PlayfieldState::PLAYFIELD_LENGTH * PlayfieldState::PLAYFIELD_EXPAND_SIZE].to_vec();
        }
    }

    pub fn update(&mut self) {
        // Pre-compute the playfield on register change. 
        let mut field = self.pf0_lookup[(self.pf0/16) as usize].clone();
        field.append(&mut self.pf1_lookup[(self.pf1) as usize].clone());
        field.append(&mut self.pf2_lookup[(self.pf2) as usize].clone());

        self.pf_lookup = field.clone();

        // If right half is reversed, then reverse it.
        if 0 != self.ctrlpf & 0x1 {
            field.reverse();
        }

        self.pf_lookup.append(&mut field.clone());
    }

    fn get_playfield_scan(&self) -> Vec<bool> {
        self.pf_lookup.clone()
    }

    pub fn update_pf0(&mut self, data: u8) {
        self.pf0 = data;
        self.update();
    }

    pub fn update_pf1(&mut self, data: u8) {
        self.pf1 = data;
        self.update();
    }

    pub fn update_pf2(&mut self, data: u8) {
        self.pf2 = data;
        self.update();
    }

    pub fn update_ctrlpf(&mut self, data: u8) {
        self.ctrlpf = data;
        self.update();
    }
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
        cxmp:(u8, u8),
        cxpfb:(u8, u8),
        cxmfb:(u8, u8),
        cxblpf:u8,
        cxppmm :u8,
}

impl CollisionState {
    pub fn new() -> Self {
        Self {
            cxmp:(0, 0),
            cxpfb:(0, 0),
            cxmfb:(0, 0),
            cxblpf: 0,
            cxppmm: 0,
        }
    }

    fn clear(&mut self) {
        self.cxmp = (0, 0);
        self.cxpfb = (0, 0);
        self.cxmfb = (0, 0);
        self.cxblpf =  0;
        self.cxppmm =  0;
    }

    pub fn update_collisions(&mut self, p0:bool, p1:bool, m0:bool, m1:bool, bl:bool, pf:bool) {

        if m0{
            if p1 {
                self.cxmp.0 |= 0x80; // m0 & p1
            }
            if pf {
                self.cxmfb.0 |= 0x80; // m0 & pf
            }
            if bl {
                self.cxmfb.0 |= 0x40; // m0 & bl
            }
            if m1 {
                self.cxppmm |= 0x40; // m0 & m1
            }
            if p0 {
                self.cxmp.0 |= 0x40; // m0 & p0
            }
        }

        if m1{
            if pf {
                self.cxmfb.1 |= 0x80; // m1 & pf
            }
            if bl {
                self.cxmfb.1 |= 0x40; // m1 & bl
            }
            if p0 {
                self.cxmp.1 |= 0x80; // m1 & p0
            }
            if p1 {
                self.cxmp.1 |= 0x40; // m1 & p1
            }
        }

        if bl{
            if pf {
                self.cxblpf |= 0x80; // bl & pf
            }
            if p0 {
                self.cxpfb.0 |= 0x40; // bl & p0
            }
            if p1 {
                self.cxpfb.1 |= 0x40; // bl & p1
            }
        }

        if p0{
            if pf {
                self.cxpfb.0 |= 0x80; // p0 & pf
            }
            if p1 {
                self.cxppmm |= 0x80; // p0 & p1
            }
        }

        if p1 & pf{
            self.cxpfb.1 |= 0x80; // p1 & pf
        }
    }

    pub fn get_cxmp_0(&mut self, ) -> u8 {
        self.cxmp.0
    }

    pub fn get_cxmp_1(&mut self, ) -> u8 {
        self.cxmp.1
    }

    pub fn get_cxpfb_0(&mut self, ) -> u8 {
        self.cxpfb.0
    }

    pub fn get_cxpfb_1(&mut self, ) -> u8 {
        self.cxpfb.1
    }

    pub fn get_cxmfb_0(&mut self, ) -> u8 {
        self.cxmfb.0
    }

    pub fn get_cxmfb_1(&mut self, ) -> u8 {
        self.cxmfb.1
    }

    pub fn get_cxblpf(&mut self, ) -> u8 {
        self.cxblpf
    }

    pub fn get_cxppmm(&mut self, ) -> u8 {
        self.cxppmm
    }
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
    pub vsync_debug_output_clock:clocks::ClockType,
    screen_start_clock:clocks::ClockType,
    paddle_start_clock:clocks::ClockType,
    last_screen_update_clock:clocks::ClockType,
    next_line:LineState,
    is_vsync:bool,
    is_blank:bool,
    is_input_latched:bool,
    is_update_time:bool,

    colours: Colours,

    display_lines: Vec< Vec<display::Colour> >,

    collision_state: CollisionState,
    playfield_state: PlayfieldState,
}

impl Stella {
    pub const FRAME_WIDTH:u16 = 160;
    pub const FRAME_HEIGHT:u16 = 280;
    pub const HORIZONTAL_BLANK:u16 = 68;
    pub const HORIZONTAL_TICKS:clocks::ClockType = (Stella::FRAME_WIDTH + Stella::HORIZONTAL_BLANK) as clocks::ClockType;
    pub const INPUT_45_LATCH_MASK:u8 = 0x40;
    pub const BLANK_PADDLE_RECHARGE:u8 = 0x80;
    pub const BLANK_MASK:u8 = 0x2;
    pub const BLANK_ON:u8 = 0x2;
    pub const BLANK_OFF:u8 = 0x0;

    pub const PF_PRIORITY:u8 = 0x4;

    pub const VBLANK_LINES:u16 = 37;
    pub const OVERSCAN_LINES:u16 = 30;

    pub const START_DRAW_Y:u16 = 0;
    pub const END_DRAW_Y:u16 = Stella::VBLANK_LINES + Stella::FRAME_HEIGHT + Stella::OVERSCAN_LINES;

    pub fn new() -> Self {
        let mut colours = Colours::new();
        colours.load("palette.dat");

        Self { 
            vsync_debug_output_clock: 0,
            screen_start_clock: 0,
            paddle_start_clock: 0,
            last_screen_update_clock: 0,
            next_line: LineState::new(),
            is_vsync:false,
            is_blank:false,
            is_input_latched:false,
            is_update_time:false,
            colours: colours,
            display_lines: vec![vec![display::Colour::new(0, 0, 0); Stella::FRAME_WIDTH as usize]; (Stella::END_DRAW_Y - Stella::START_DRAW_Y + 1) as usize],
            collision_state: CollisionState::new(),
            playfield_state: PlayfieldState::new(),
        }
    }

    pub fn write(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        if false == self.is_blank {
//            self.screen_scan(clock, &self.next_line, &mut self.display_lines);
            self.screen_scan(clock);
        }

        // TODO

        self.write_functions(clock, address, data);
    }
    pub fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        let result;

        match address & 0xF {
            0x0 => { result = self.collision_state.get_cxmp_0() }
            0x1 => { result = self.collision_state.get_cxmp_1() }
            0x2 => { result = self.collision_state.get_cxpfb_0() }
            0x3 => { result = self.collision_state.get_cxpfb_1() }
            0x4 => { result = self.collision_state.get_cxmfb_0() }
            0x5 => { result = self.collision_state.get_cxmfb_1() }
            0x6 => { result = self.collision_state.get_cxblpf(); }
            0x7 => { result = self.collision_state.get_cxppmm(); }
// TODO: Inputs
//            0x8 => {
//                result = self._inpt[0];
//                // paddle0 stuff
//            }
//            0x9 => {
//                result = self._inpt[1];
//                // paddle1 stuff
//            }
//            0xA => {
//                // paddle3 stuff
//                result = self._inpt[2];
//            }
//            0xB => { result = self.inputs.get_input7(); }
//            0xC => { result = self.inputs.get_input7(); }
//            0xD => { result = self._inpt[5]; }
            _ => { println!("Stella read: {:X}", address); 
                result = 0;
            }
        }

        result
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
                println!("Stella write not supported 0x{:X}", address & 0x3F);
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
                self.vsync_debug_output_clock =  clock.ticks;
                // TODO: Check 'actual' calc (mod of negative seems inconsistent) self.screen_start_clock = clock.ticks.wrapping_sub(Stella::HORIZONTAL_TICKS).wrapping_add((Stella::HORIZONTAL_TICKS.wrapping_sub(clock.ticks).wrapping_add(self.screen_start_clock)) % Stella::HORIZONTAL_TICKS);
                self.screen_start_clock = clock.ticks;
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
        self.next_line.p_colour.0 = self.colours.get_color(data);
    }

    fn write_colump1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.p_colour.1 = self.colours.get_color(data);
    }

    fn write_colupf(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.playfield_colour = self.colours.get_color(data);
    }

    fn write_colubk(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.background_colour = self.colours.get_color(data);
    }

    fn write_ctrlpf(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO

        self.next_line.ctrlpf  = data;
        self.playfield_state.update_ctrlpf(data);
        // TODO
//            self.ball.update_ctrlpf(data)
    }

    fn write_refp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_refp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
    }

    fn write_pf0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.playfield_state.update_pf0(data);
    }

    fn write_pf1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.playfield_state.update_pf1(data);
    }

    fn write_pf2(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.playfield_state.update_pf2(data);
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

//    pub fn screen_scan(&mut self, clock: &mut clocks::Clock, next_line:& LineState, display_lines: &mut Vec< Vec<display::Colour> >) {
    pub fn screen_scan(&mut self, clock: &mut clocks::Clock) {

      let future_pixels = 1;
    
      let last_screen_pos = (self.last_screen_update_clock - self.screen_start_clock) as u16;
      let screen_pos:u16 = ((clock.ticks - self.screen_start_clock) as u16).wrapping_add(future_pixels);
    
      let y_start = (last_screen_pos/Stella::HORIZONTAL_TICKS as u16) - Stella::START_DRAW_Y;
      let y_stop  = (screen_pos/Stella::HORIZONTAL_TICKS as u16) - Stella::START_DRAW_Y;

      if y_stop < (Stella::END_DRAW_Y - Stella::START_DRAW_Y) {

        let priority_ctrl = 0 == self.next_line.ctrlpf & Stella::PF_PRIORITY;
        let nl_p_colour0  = self.next_line.p_colour.0;
        let nl_p_colour1  = self.next_line.p_colour.1;
        let nl_pf_colour  = self.next_line.playfield_colour;
        let nl_bg_colour  = self.next_line.background_colour;

        let p0_scan = vec![false; Stella::FRAME_WIDTH as usize]; //self.p0_state.get_player_scan();
        let p1_scan = vec![false; Stella::FRAME_WIDTH as usize]; //self.p1_state.get_player_scan();
        let pf_scan = self.playfield_state.get_playfield_scan();
        let m0_scan = vec![false; Stella::FRAME_WIDTH as usize]; //self.missile0.get_missile_scan();
        let m1_scan = vec![false; Stella::FRAME_WIDTH as usize]; //self.missile1.get_missile_scan();
        let bl_scan = vec![false; Stella::FRAME_WIDTH as usize]; //self.ball.get_ball_scan();

        let mut x_start = 0;
        if (last_screen_pos % Stella::HORIZONTAL_TICKS as u16) >= Stella::HORIZONTAL_BLANK {
            x_start = (last_screen_pos % Stella::HORIZONTAL_TICKS as u16) - Stella::HORIZONTAL_BLANK;
        }

        let mut last_x_stop = 0;
        if (screen_pos % Stella::HORIZONTAL_TICKS as u16) >= Stella::HORIZONTAL_BLANK {
          last_x_stop = screen_pos % Stella::HORIZONTAL_TICKS as u16 - Stella::HORIZONTAL_BLANK;
        }

        for y in y_start as u16 .. (y_stop+1) as u16 {
    
          let x_stop;
          if y == y_stop {
            x_stop = last_x_stop;
          } else {
            x_stop = Stella::FRAME_WIDTH - 1;
          }
    
          let current_y_line = &mut self.display_lines[y as usize];
          for x in x_start as usize ..x_stop as usize  {
    
            let pf = pf_scan[x];
            let bl = bl_scan[x];
            let m1 = m1_scan[x];
            let p1 = p1_scan[x];
            let m0 = m0_scan[x];
            let p0 = p0_scan[x];

            // Priorities (bit 2 set):  Priorities (bit 2 clear):
            //  PF, BL                   P0, M0
            //  P0, M0                   P1, M1
            //  P1, M1                   PF, BL
            //  BK                       BK
            let mut pixel_colour = nl_bg_colour;
            let mut hits = 0;
            if priority_ctrl {
              if pf || bl {
                  pixel_colour = nl_pf_colour;
                  hits += bl as u8 + pf as u8;
              }
              if p1 || m1 {
                  pixel_colour = nl_p_colour1;
                  hits += m1 as u8 + p1 as u8;
              }
              if p0 || m0 {
                  pixel_colour = nl_p_colour0;
                  hits += m0 as u8 + p0 as u8;
              }
            } else {
              if p1 || m1 {
                  pixel_colour = nl_p_colour1;
                  hits += m1 as u8 + p1 as u8;
              }
              if p0 || m0 {
                  pixel_colour = nl_p_colour0;
                  hits += m0 as u8 + p0 as u8;
              }
              if pf || bl {
                  pixel_colour = nl_pf_colour;
                  hits += bl as u8 + pf as u8;
              }
            }

            if hits > 1 {
                self.collision_state.update_collisions(p0, p1, m0, m1, bl, pf);
            }

//       Display scan 'start position'.
//            ps0 = self.p0_state._pos_start
//            ps1 = self.p1_state._pos_start
//            if x == ps0:
//                pixel_colour = self._colors.get_color(2)
//            if x == ps1:
//                pixel_colour = self._colors.get_color(3)
//
            current_y_line[x] = pixel_colour;
          }

          x_start = 0
        }
      }

      self.last_screen_update_clock = clock.ticks + future_pixels as u64;
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

impl io::DebugClock for Stella{
    fn debug_clock(&mut self) -> clocks::ClockType {
        self.vsync_debug_output_clock
    }
}

impl io::StellaIO for Stella{
    fn export(&mut self) -> bool {
        true
    }

    fn generate_display(&mut self, buffer: &mut [u8]) {
        let mut index = 0;
        for y in 0..Stella::FRAME_HEIGHT {
            let display_line = &self.display_lines[(y + Stella::START_DRAW_Y) as usize];
            for x in display_line {
                x.convert_rgb888(&mut buffer[index..(index + display::SDLUtility::bytes_per_pixel() as usize)]);
                index += display::SDLUtility::bytes_per_pixel() as usize;
            }
        }
    }
}
