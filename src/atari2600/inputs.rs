use sdl2::keyboard; // Keycode
use sdl2::event; // Keycode

#[derive(Clone,Copy)]
pub struct Input {
    pub swcha:u8,
    pub swchb:u8,
    pub input0:u8,
    pub input1:u8,
    pub input2:u8,
    pub input3:u8,
    pub input4:u8,
    pub input5:u8,
    pub quit:u8,
}

impl Input {
    const INPUT_MASK:u8 = 0x80; // 'INP0-INP5' set data bit '7' (0-6 are ignored).  
                                // 'I0-I3' can be grouned by software and 'I4-I5' can be configured to latch via software, but this isn't emulated.
    pub  fn new() -> Self {
        Self {
            swcha: 0xFF,
            swchb: 0x3F,
            input0: 0xFF,
            input1: 0xFF,
            input2: 0xFF,
            input3: 0xFF,
            input4: 0xFF,
            input5: 0xFF,
            quit: 0x0,
        }
    }
}

#[derive(Clone,Copy)]
pub struct Joystick {
    pub input:Input,
}

impl Joystick {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
        }
    }

    pub fn set_input (value:bool, initial:&mut u8, mask:u8) {
        if value { 
            *initial &= !mask;
        } else {
            *initial |= mask;
        }
    }

    pub fn toggle_input (initial:&mut u8, mask:u8) {
        *initial ^= mask;
    }

    pub fn j1_up    (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swcha, 0x10); }
    pub fn j1_down  (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swcha, 0x20); }
    pub fn j1_left  (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swcha, 0x40); }
    pub fn j1_right (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swcha, 0x80); }
    pub fn j1_fire  (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.input4, Input::INPUT_MASK); }

    pub fn j2_up    (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swcha, 0x01); }
    pub fn j2_down  (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swcha, 0x02); }
    pub fn j2_left  (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swcha, 0x04); }
    pub fn j2_right (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swcha, 0x08); }
    pub fn j2_fire  (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.input5, Input::INPUT_MASK); }

    pub fn select   (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swchb, 0x01); }
    pub fn reset    (&mut self, value:bool) { Joystick::set_input (value, &mut self.input.swchb, 0x02); }
    pub fn p0_difficulty (&mut self) { println!("difficulty 0"); Joystick::toggle_input (&mut self.input.swchb, 0x40); }
    pub fn p1_difficulty (&mut self) { println!("difficulty 1"); Joystick::toggle_input (&mut self.input.swchb, 0x80); }
}

pub struct UserInput {
}

impl UserInput {
    const KEY_1_UP:keyboard::Keycode     = keyboard::Keycode::Up;
    const KEY_1_DOWN:keyboard::Keycode   = keyboard::Keycode::Down;
    const KEY_1_LEFT:keyboard::Keycode   = keyboard::Keycode::Left;
    const KEY_1_RIGHT:keyboard::Keycode  = keyboard::Keycode::Right;
    const KEY_1_FIRE:keyboard::Keycode   = keyboard::Keycode::RCtrl;

    const KEY_2_UP:keyboard::Keycode     = keyboard::Keycode::I;
    const KEY_2_DOWN:keyboard::Keycode   = keyboard::Keycode::K;
    const KEY_2_LEFT:keyboard::Keycode   = keyboard::Keycode::J;
    const KEY_2_RIGHT:keyboard::Keycode  = keyboard::Keycode::L;
    const KEY_2_FIRE:keyboard::Keycode   = keyboard::Keycode::Space;

    const KEY_RESET:keyboard::Keycode  = keyboard::Keycode::R;
    const KEY_SELECT:keyboard::Keycode  = keyboard::Keycode::S;
    const KEY_P0_DIFFICULTY:keyboard::Keycode = keyboard::Keycode::Num1;
    const KEY_P1_DIFFICULTY:keyboard::Keycode = keyboard::Keycode::Num2;
    const KEY_QUIT:keyboard::Keycode   = keyboard::Keycode::Escape;

    pub fn print_keys() {
        println!("Key mappings (Joystick 1):");
        println!("Up: {}, Down: {}, Left: {}, Right: {}", UserInput::KEY_1_UP, UserInput::KEY_1_DOWN, UserInput::KEY_1_LEFT, UserInput::KEY_1_RIGHT);
        println!("Fire: {}", UserInput::KEY_1_FIRE);
        println!("Key mappings (Joystick 2):");
        println!("Up: {}, Down: {}, Left: {}, Right: {}", UserInput::KEY_2_UP, UserInput::KEY_2_DOWN, UserInput::KEY_2_LEFT, UserInput::KEY_2_RIGHT);
        println!("Fire: {}", UserInput::KEY_2_FIRE);
        println!("Reset: {}", UserInput::KEY_RESET);
        println!();
        println!("Quit: {}", UserInput::KEY_QUIT);
    }

    // Return 'true' if handled, otherwise 'false' (ie quit)
    pub fn handle_events(event:event::Event, joystick:&mut Joystick) -> bool {
        match event {
            event::Event::Quit { .. } | 
            event::Event::KeyDown { keycode: Some(UserInput::KEY_QUIT), ..  } => { return false }

            event::Event::KeyDown { keycode: Some(UserInput::KEY_1_UP), .. }     => { joystick.j1_up(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_1_DOWN), .. }   => { joystick.j1_down(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_1_LEFT), .. }   => { joystick.j1_left(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_1_RIGHT), .. }  => { joystick.j1_right(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_1_FIRE), .. }   => { joystick.j1_fire(true); }

            event::Event::KeyDown { keycode: Some(UserInput::KEY_2_UP), .. }     => { joystick.j2_up(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_2_DOWN), .. }   => { joystick.j2_down(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_2_LEFT), .. }   => { joystick.j2_left(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_2_RIGHT), .. }  => { joystick.j2_right(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_2_FIRE), .. }   => { joystick.j2_fire(true); }

            event::Event::KeyDown { keycode: Some(UserInput::KEY_RESET), .. }  => { joystick.reset(true); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_SELECT), .. }  => { joystick.select(true); }

            event::Event::KeyUp { keycode: Some(UserInput::KEY_1_UP), .. }     => { joystick.j1_up(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_1_DOWN), .. }   => { joystick.j1_down(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_1_LEFT), .. }   => { joystick.j1_left(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_1_RIGHT), .. }  => { joystick.j1_right(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_1_FIRE), .. }   => { joystick.j1_fire(false); }

            event::Event::KeyUp { keycode: Some(UserInput::KEY_2_UP), .. }     => { joystick.j2_up(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_2_DOWN), .. }   => { joystick.j2_down(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_2_LEFT), .. }   => { joystick.j2_left(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_2_RIGHT), .. }  => { joystick.j2_right(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_2_FIRE), .. }   => { joystick.j2_fire(false); }

            event::Event::KeyUp { keycode: Some(UserInput::KEY_RESET), .. }  => { joystick.reset(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_SELECT), .. }  => { joystick.select(false); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_P0_DIFFICULTY), .. } => { joystick.p0_difficulty(); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_P1_DIFFICULTY), .. } => { joystick.p1_difficulty(); }

            _ => {return true}
        }

        true
    }
}
