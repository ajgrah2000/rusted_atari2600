use super::super::io;
use super::super::clocks;
use super::super::inputs;
use super::super::audio::tiasound;
use super::display;
use std;
use std::io::BufRead;

use super::super::audio::soundchannel;

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
        enabl:u8,
        enabl_old:u8,
        vdelbl:u8,
        resbl:u8,
        ctrlpf:u8,

        x_min:u16,
        x_max:u16,

        enabled:bool,

        scan_line: Vec<bool>,
}

impl BallState {

    fn new() -> Self {
        Self {
            enabl:     0,
            enabl_old:  0,
            vdelbl:    0,
            resbl:     0,
            ctrlpf:    0,

            x_min:     0,
            x_max:     0,

            enabled:   false,

            scan_line: vec![false; Stella::FRAME_WIDTH as usize],
        }
    }

    fn update(&mut self) {
        if 0 == (self.vdelbl & 0x1) {
            self.enabled = 0 != (self.enabl & 0x02);
        } else {
            self.enabled = 0 != (self.enabl_old & 0x02);
        }

        let width = 1 << ((self.ctrlpf & 0x30) >> 4);

        self.x_min = (self.resbl as u16).wrapping_sub(Stella::HORIZONTAL_BLANK);
        self.x_max = (self.resbl as u16).wrapping_sub(Stella::HORIZONTAL_BLANK).wrapping_add(width);

        self.calc_ball_scan()
    }

    fn update_resbl(&mut self, data:u8) {
        self.resbl = data;
        self.update();
    }
        
    fn update_enabl_old(&mut self, data:u8) {
        self.enabl_old = data;
        self.update();
    }

    fn update_enabl(&mut self, data:u8) {
        self.enabl = data;
        self.update();
    }

    fn update_vdelbl(&mut self, data:u8) {
        self.vdelbl = data;
        self.update();
    }

    fn update_ctrlpf(&mut self, data:u8) {
        self.ctrlpf = data;
        self.update();
    }

    fn calc_ball_scan(&mut self) {
        // Calculate an entire scanline for the ball, re-calculated on
        // parameter change. 
        // Default scan to false.
        self.scan_line = vec![false; Stella::FRAME_WIDTH as usize];

        if self.enabled {
            for x in self.x_min..self.x_max {
               self.scan_line[(x % Stella::FRAME_WIDTH) as usize] = true;
            }
        }
    }

    fn get_ball_scan(&self) -> Vec<bool> {
        self.scan_line.clone()
    }
}

pub struct MissileState {
    nusiz:u8,
    enam:u8,
    resm:u8,
    
    // Derived state data (nominally generated during update)
    number:u8,
    gap:u8,
    
    // Default scan to false.
    scan_line: Vec<bool>,
}

impl MissileState {

    fn new() -> Self {
        Self {
            nusiz: 0,
            enam: 0,
            resm: 0,

            // Derived state data (nominally generated during update)
            number: 0,
            gap: 0,

            // Default scan to false.
            scan_line: vec![false; Stella::FRAME_WIDTH as usize],
        }
    }

    fn update(&mut self) {
        // Missiles ignore scaling options.
        let (number, size, gap) = Stella::nusize(self.nusiz);
        self.number = number;
        self.gap    = gap;

        if self.resm < Stella::HORIZONTAL_BLANK as u8{
            self.resm = Stella::HORIZONTAL_BLANK as u8;
        }

        self.calc_missile_scan();
    }

    fn update_nusiz(&mut self, data:u8) {
        self.nusiz = data;
        self.update();
    }

    fn update_resm(&mut self, data:u8) {
        self.resm = data;
        self.update();
    }

    fn update_enam(&mut self, data:u8) {
        self.enam = data;
        self.update();
    }

    fn calc_missile_scan(&mut self) {
        // Pre-calculate an entire scan line, as update is called relatively
        // infrequently. 
        self.scan_line = vec![false; Stella::FRAME_WIDTH as usize];

        if 0 != self.enam & 0x02 {
            for n in 0..self.number {
                // Uses same stretching as 'ball'
                let width = 1 << ((self.nusiz & 0x30) >> 4);
                // Uses similar position to 'player'
                for i in 0..width {
                    let x = (i +self.resm + n*self.gap * 8 - Stella::HORIZONTAL_BLANK as u8) % Stella::FRAME_WIDTH as u8;
                    self.scan_line[x as usize] = true;
                }
            }
        }
    }

    fn get_missile_scan(&self) -> Vec<bool> {
        self.scan_line.clone()
    }
}

pub struct PlayerState {
    nusiz:u8,
    p:u8,
    p_old:u8,
    refp:u8,
    resp:u8,
    vdelp:u8,
    reflect:u8,

    // Derived state data (nominally generated during update)
    grp:u8,
    number:u8,
    size:u8,
    gap:u8,

    pos_start:u16,

    scan_line: Vec<bool>,

    player_scan_unshifted: Vec<Vec<Vec<Vec<Vec<Vec<bool>>>>>>,
}

impl PlayerState {
    // Only 1,2,3 required, but 0..3 calculated
    const NUMBER_RANGE:usize = 4;

    // Only 1,2,4 required, but 0..4 calculated
    const SIZE_RANGE:usize = 5;

    // Gaps are 0, 2, 4, 8
    const GAP_RANGE:usize = 9;

    const REFLECT_RANGE:usize = 2;

    const GRAPHIC_RANGE:usize = 256;

    fn new() -> Self {
        let mut instance = Self {
            nusiz: 0,
            p: 0,
            p_old: 0,
            refp: 0,
            resp: 0,
            vdelp: 0,
            reflect:0,

            // Derived state data (nominally generated during update)
            grp: 0,
            number: 0,
            size: 0,
            gap: 0,

            pos_start: 0,

            scan_line: vec![false; Stella::FRAME_WIDTH as usize],

            player_scan_unshifted: vec![vec![vec![vec![vec![vec![false; Stella::FRAME_WIDTH as usize]; PlayerState::GRAPHIC_RANGE]; PlayerState::REFLECT_RANGE]; PlayerState::GAP_RANGE]; PlayerState::SIZE_RANGE]; PlayerState::NUMBER_RANGE],
        };

        instance.pre_calc_player();

        instance
    }

    fn update_nusiz(&mut self, data:u8) {
        self.nusiz = data;
        self.update();
    }

    fn update_resp(&mut self, data:u8) {
        self.resp = data;
        self.update();
    }

    fn update_refp(&mut self, data:u8) {
        self.refp = data;
        self.update();
    }

    fn update_p(&mut self, data:u8) {
        self.p = data;
        self.update();
    }

    fn update_p_old(&mut self, data:u8) {
        self.p_old = data;
        self.update();
    }

    fn update_vdelp(&mut self, data:u8) {
        self.vdelp = data;
        self.update();
    }

    fn pre_calc_player(&mut self) {
        // Precalculate all number, gap, size, graphic combinations.

        // Create enough empty lists to allow direct indexing.
        for number in [1,2,3] {
            for size in [1,2,4] {
                for gap in [0,2,4,8] {
                    for reflect in 0..2 {
                        for g in 0..PlayerState::GRAPHIC_RANGE {
                            // Create the 8-bit 'graphic'
                            let mut graphic = vec![false; 8];
                            for i in 0..graphic.len() {
                                if 0 != (g >> i) & 0x01 {
                                    graphic[i] = true;
                                }
                            }

                            if 0 != reflect {
                                graphic.reverse();
                            }

                            // Scale the graphic, so each pixel is 'size' big
                            let mut scaled_graphic = vec![false; graphic.len() * size as usize];
                            for i in 0..graphic.len() {
                                for s in 0..size {
                                    scaled_graphic[(i * size + s) as usize] = graphic[i];
                                }
                            }

                            let mut scan = vec![false; Stella::FRAME_WIDTH as usize];
                            for n in 0..number {
                                let offset = n*gap*8;
                                for i in 0..scaled_graphic.len() {
                                    scan[offset + i] = scaled_graphic[i];
                                }
                            }

                            self.player_scan_unshifted[number][size][gap][reflect][g] = scan;
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        if 0 == (self.vdelp & 0x1) {
            self.grp = self.p;
        }
        else {
            self.grp = self.p_old;
        }

        if 0 == self.grp {
            self.scan_line = vec![false; 160];
        }
        else {
            let (number, size, gap) = Stella::nusize(self.nusiz);
            self.number = number;
            self.size   = size;
            self.gap    = gap;

            if self.resp < Stella::HORIZONTAL_BLANK as u8{
                self.resp = Stella::HORIZONTAL_BLANK as u8;
            }
            if (self.refp & 0x8) == 0 {
                self.reflect = 1;
            }
            else {
                self.reflect = 0;
            }

            // TODO: Check if 'start' can go negative.
            self.pos_start = (self.resp as u16).wrapping_sub(Stella::HORIZONTAL_BLANK).wrapping_add(self.size as u16/2);
            self.calc_player_scan();
        }
    }

    fn calc_player_scan(&mut self) {
        // Rotate the scan.
        let rotation = Stella::FRAME_WIDTH-self.pos_start;
        let scan = &self.player_scan_unshifted[self.number as usize][self.size as usize][self.gap as usize][self.reflect as usize][self.grp as usize];
        self.scan_line = scan[rotation as usize ..].to_vec();
        self.scan_line.append(&mut scan[..rotation as usize ].to_vec());
    }
                            
        
    fn get_player_scan(&self) -> Vec<bool> {
        self.scan_line.clone()
    }
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

    pub fn get_cxmp_0(&mut self) -> u8 {
        self.cxmp.0
    }

    pub fn get_cxmp_1(&mut self) -> u8 {
        self.cxmp.1
    }

    pub fn get_cxpfb_0(&mut self) -> u8 {
        self.cxpfb.0
    }

    pub fn get_cxpfb_1(&mut self) -> u8 {
        self.cxpfb.1
    }

    pub fn get_cxmfb_0(&mut self) -> u8 {
        self.cxmfb.0
    }

    pub fn get_cxmfb_1(&mut self) -> u8 {
        self.cxmfb.1
    }

    pub fn get_cxblpf(&mut self) -> u8 {
        self.cxblpf
    }

    pub fn get_cxppmm(&mut self) -> u8 {
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

    pub fn get_colour(&self, colour:u8) -> display::Colour {
        self.colours[colour as usize >> 1]
    }
}

pub struct Stella {
    pub tiasound: tiasound::TiaSound,

    input:inputs::Input,
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
    p0_state: PlayerState,
    p1_state: PlayerState,
    missile0: MissileState,
    missile1: MissileState,
    ball: BallState,

    scanline_debug: bool,
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

    pub fn new(scanline_debug:bool, realtime:bool, pal_palette:bool) -> Self {
        let mut colours = Colours::new();
        colours.load(if pal_palette {"palette_pal.dat"} else {"palette_ntsc.dat"});

        Self { 
            tiasound:tiasound::TiaSound::new(realtime),
            input:inputs::Input::new(),
            vsync_debug_output_clock: 0,
            screen_start_clock: 0,
            paddle_start_clock: 0,
            last_screen_update_clock: 0,
            next_line: LineState::new(),
            is_vsync:false,
            is_blank:true,
            is_input_latched:false,
            is_update_time:false,
            colours: colours,
            display_lines: vec![vec![display::Colour::new(0, 0, 0); Stella::FRAME_WIDTH as usize]; (Stella::END_DRAW_Y - Stella::START_DRAW_Y + 1) as usize],
            collision_state: CollisionState::new(),
            playfield_state: PlayfieldState::new(),
            p0_state: PlayerState::new(),
            p1_state: PlayerState::new(),
            missile0: MissileState::new(),
            missile1: MissileState::new(),
            ball: BallState::new(),
            scanline_debug: scanline_debug,
        }
    }

    pub fn write(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        // TODO
        // tiasound and update scans

        if false == self.is_blank {
            self.screen_scan(clock);
        }

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
            0xB => { result = self.input.input7; }
            0xC => { result = self.input.input7; }
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
            0x15 => {self.tiasound.write_audio_ctrl_0(clock, address, data); }
            0x16 => {self.tiasound.write_audio_ctrl_1(clock, address, data); }
            0x17 => {self.tiasound.write_audio_freq_0(clock, address, data); }
            0x18 => {self.tiasound.write_audio_freq_1(clock, address, data); }
            0x19 => {self.tiasound.write_audio_vol_0(clock, address, data); }
            0x1A => {self.tiasound.write_audio_vol_1(clock, address, data); }
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
//                println!("Stella write not supported 0x{:X}", address & 0x3F);
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
        self.p0_state.update_nusiz(data);
        self.missile0.update_nusiz(data);
    }

    fn write_nusiz1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.p1_state.update_nusiz(data);
        self.missile1.update_nusiz(data)
    }

    fn write_colump0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.p_colour.0 = self.colours.get_colour(data);
    }

    fn write_colump1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.p_colour.1 = self.colours.get_colour(data);
    }

    fn write_colupf(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.playfield_colour = self.colours.get_colour(data);
    }

    fn write_colubk(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.background_colour = self.colours.get_colour(data);
    }

    fn write_ctrlpf(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.ctrlpf  = data;
        self.playfield_state.update_ctrlpf(data);
        self.ball.update_ctrlpf(data)
    }

    fn write_refp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.p0_state.update_refp(data);
    }

    fn write_refp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.p1_state.update_refp(data);
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
        self.p0_state.update_resp(((clock.ticks + 5 - self.screen_start_clock) % Stella::HORIZONTAL_TICKS) as u8);
    }

    fn write_resp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.p1_state.update_resp(((clock.ticks + 5 - self.screen_start_clock) % Stella::HORIZONTAL_TICKS) as u8);
    }

    fn write_resm0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.missile0.update_resm(((clock.ticks + 4 - self.screen_start_clock) % Stella::HORIZONTAL_TICKS) as u8);
    }

    fn write_resm1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.missile1.update_resm(((clock.ticks + 4 - self.screen_start_clock) % Stella::HORIZONTAL_TICKS) as u8);
    }

    fn write_resbl(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.ball.update_resbl(((clock.ticks.wrapping_add(4).wrapping_sub(self.screen_start_clock)) % Stella::HORIZONTAL_TICKS) as u8);
    }

    fn write_grp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.p0_state.update_p(data);
        self.p1_state.update_p_old(self.p1_state.p);
    }

    fn write_grp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.p1_state.update_p(data);
        self.p0_state.update_p_old(self.p0_state.p);
        self.ball.update_enabl_old(self.ball.enabl);
    }

    fn write_enam0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.missile0.update_enam(data);
    }

    fn write_enam1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.missile1.update_enam(data);
    }

    fn write_enabl(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.ball.update_enabl(data);
    }

    fn write_hmp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.hmp.0 = data;
    }

    fn write_hmp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.hmp.1 = data;
    }

    fn write_hmm0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.hmm.0 = data;
    }

    fn write_hmm1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.hmm.1 = data;
    }

    fn write_hmbl(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.hmbl = data;
    }

    fn write_hmove(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.hmove();
    }

    fn write_hclr(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.next_line.hmp.0 = 0;
        self.next_line.hmp.1 = 0;
        self.next_line.hmm.0 = 0;
        self.next_line.hmm.1 = 0;
        self.next_line.hmbl = 0;
    }

    fn write_vdelp0(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.p0_state.update_vdelp(data);
    }

    fn write_vdelp1(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.p1_state.update_vdelp(data);
    }

    fn write_vdelbl(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.ball.update_vdelbl(data);
    }

    fn write_cxclr(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.collision_state.clear();
    }

    pub fn screen_scan(&mut self, clock: &mut clocks::Clock) {

      let future_pixels = 1;
    
      let last_screen_pos = self.last_screen_update_clock - self.screen_start_clock;
      let screen_pos = (clock.ticks - self.screen_start_clock).wrapping_add(future_pixels as clocks::ClockType);
    
      let y_start = (last_screen_pos/Stella::HORIZONTAL_TICKS as clocks::ClockType) as u16 - Stella::START_DRAW_Y;
      let y_stop  = (screen_pos/Stella::HORIZONTAL_TICKS as clocks::ClockType) as u16 - Stella::START_DRAW_Y;

      if y_stop < (Stella::END_DRAW_Y - Stella::START_DRAW_Y) {

        let priority_ctrl = 0 == self.next_line.ctrlpf & Stella::PF_PRIORITY;
        let nl_p_colour0  = self.next_line.p_colour.0;
        let nl_p_colour1  = self.next_line.p_colour.1;
        let nl_pf_colour  = self.next_line.playfield_colour;
        let nl_bg_colour  = self.next_line.background_colour;

        let p0_scan = self.p0_state.get_player_scan();
        let p1_scan = self.p1_state.get_player_scan();
        let pf_scan = self.playfield_state.get_playfield_scan();
        let m0_scan = self.missile0.get_missile_scan();
        let m1_scan = self.missile1.get_missile_scan();
        let bl_scan = self.ball.get_ball_scan();

        let mut x_start = 0;
        if ((last_screen_pos % Stella::HORIZONTAL_TICKS as clocks::ClockType) as u16) >= Stella::HORIZONTAL_BLANK {
            x_start = (last_screen_pos % Stella::HORIZONTAL_TICKS as clocks::ClockType) as u16 - Stella::HORIZONTAL_BLANK;
        }

        let mut last_x_stop = 0;
        if ((screen_pos % Stella::HORIZONTAL_TICKS as clocks::ClockType) as u16) >= Stella::HORIZONTAL_BLANK {
          last_x_stop = (screen_pos % Stella::HORIZONTAL_TICKS as clocks::ClockType) as u16 - Stella::HORIZONTAL_BLANK;
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

            if self.scanline_debug {
                // Display scan 'start position'.
                let ps0 = self.p0_state.pos_start;
                let ps1 = self.p1_state.pos_start;
                if x as u16 == ps0 {
                    pixel_colour = self.colours.get_colour(2);
                }
                if x as u16 == ps1 {
                    pixel_colour = self.colours.get_colour(3);
                }
            }

            current_y_line[x] = pixel_colour;
          }

          x_start = 0;
        }
      }

      self.last_screen_update_clock = clock.ticks + future_pixels as u64;
    }

    fn nusize(nusiz:u8) -> (u8, u8, u8) {
        // (number, size, gap)
        match nusiz & 0x7 {
            0 => { (1, 1, 0) },
            1 => { (2, 1, 2) },
            2 => { (2, 1, 4) },
            3 => { (3, 1, 2) },
            4 => { (2, 1, 8) },
            5 => { (1, 2, 0) },
            6 => { (3, 1, 4) },
            7 => { (1, 4, 0) },
            _ => { panic!("nusize: this should be impossible");},
        }
    }

    fn hmove(&mut self) {
        self.p0_state.resp  = (self.p0_state.resp.wrapping_sub(Stella::hmove_clocks(self.next_line.hmp.0) as u8)) % Stella::HORIZONTAL_TICKS as u8;
        self.p1_state.resp  = (self.p1_state.resp.wrapping_sub(Stella::hmove_clocks(self.next_line.hmp.1) as u8)) % Stella::HORIZONTAL_TICKS as u8;
        self.missile0.resm  = self.missile0.resm.wrapping_sub(Stella::hmove_clocks(self.next_line.hmm.0) as u8) % Stella::HORIZONTAL_TICKS as u8;
        self.missile1.resm  = self.missile1.resm.wrapping_sub(Stella::hmove_clocks(self.next_line.hmm.1) as u8) % Stella::HORIZONTAL_TICKS as u8;
        self.ball.resbl     = self.ball.resbl.wrapping_sub(Stella::hmove_clocks(self.next_line.hmbl) as u8) % Stella::HORIZONTAL_TICKS as u8;

        self.p0_state.update();
        self.p1_state.update();
        self.missile0.update();
        self.missile1.update();
        self.ball.update();
    }

    fn hmove_clocks(hm:u8) -> i8 {
        // hm - int8
        // Need to ensure 'hm' maintains negative when shifted.
        // 'hm >= 0x80' is negative move.
        let clock_shift = (hm as i8) >> 4;
        clock_shift
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
    fn set_inputs(&mut self, inputs: inputs::Input) {
        self.input = inputs;
    }

    fn get_next_audio_chunk(&mut self, length: u32) -> Vec<soundchannel::PlaybackType> {
        self.tiasound.get_next_audio_chunk(length)
    }

    fn step_tia_sound(&mut self, clock: &clocks::Clock) {
        self.tiasound.step(clock);
    }

    fn export(&mut self) -> bool {
        // If it's time to update, then return the current value and clear it.
        let result = self.is_update_time;
        self.is_update_time = false;
        result 
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
