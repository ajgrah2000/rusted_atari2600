use sdl2::keyboard; // Keycode
use sdl2::event; // Keycode

#[derive(Clone,Copy)]
pub struct Input {
    pub swcha:u8,
    pub swchb:u8,
    pub input7:u8,
    pub paddle0:u8,
    pub quit:u8,
}

impl Input {
    pub  fn new() -> Self {
        Self {
            swcha: 0xFF,
            swchb: 0x3F,
            input7: 0xFF,
            paddle0: 0x00,
            quit: 0x0,
        }
    }
}

#[derive(Clone,Copy)]
pub struct Joystick {
    input:Input,
}

impl Joystick {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
        }
    }

    pub fn j1_up    (&mut self, value:bool) {}
    pub fn j1_down  (&mut self, value:bool) {}
    pub fn j1_left  (&mut self, value:bool) {}
    pub fn j1_right (&mut self, value:bool) {}
    pub fn j1_fire  (&mut self, value:bool) {}
    pub fn j2_up    (&mut self, value:bool) {}
    pub fn j2_down  (&mut self, value:bool) {}
    pub fn j2_left  (&mut self, value:bool) {}
    pub fn j2_right (&mut self, value:bool) {}
    pub fn j2_fire  (&mut self, value:bool) {}
    pub fn reset    (&mut self, value:bool) {}
}

pub struct UserInput {
}

impl UserInput {
    const KEY_UP:keyboard::Keycode     = keyboard::Keycode::Up;
    const KEY_DOWN:keyboard::Keycode   = keyboard::Keycode::Down;
    const KEY_LEFT:keyboard::Keycode   = keyboard::Keycode::Left;
    const KEY_RIGHT:keyboard::Keycode  = keyboard::Keycode::Right;
    const KEY_FIRE:keyboard::Keycode   = keyboard::Keycode::Z;
    const KEY_RESET:keyboard::Keycode  = keyboard::Keycode::R;
    const KEY_QUIT:keyboard::Keycode   = keyboard::Keycode::Escape;

    pub fn print_keys() {
        println!("Key mappings (Joystick 1):");
        println!("Up: {}, Down: {}, Left: {}, Right: {}", UserInput::KEY_UP, UserInput::KEY_DOWN, UserInput::KEY_LEFT, UserInput::KEY_RIGHT);
        println!("Fire: {}", UserInput::KEY_FIRE);
        println!("Reset: {}", UserInput::KEY_RESET);
        println!();
        println!("Quit: {}", UserInput::KEY_QUIT);
    }

    // Return 'true' if handled, otherwise 'false' (ie quit)
    pub fn handle_events(event:event::Event, joystick:&mut Joystick) -> bool {
        match event {
            event::Event::Quit { .. } | 
            event::Event::KeyDown { keycode: Some(UserInput::KEY_QUIT), ..  } => { return false }

            event::Event::KeyDown { keycode: Some(UserInput::KEY_UP), .. }     => { joystick.j1_up(false); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_DOWN), .. }   => { joystick.j1_down(false); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_LEFT), .. }   => { joystick.j1_left(false); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_RIGHT), .. }  => { joystick.j1_right(false); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_FIRE), .. }   => { joystick.j1_fire(false); }
            event::Event::KeyDown { keycode: Some(UserInput::KEY_RESET), .. }  => { joystick.reset(false); }

            event::Event::KeyUp { keycode: Some(UserInput::KEY_UP), .. }     => { joystick.j1_up(true); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_DOWN), .. }   => { joystick.j1_down(true); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_LEFT), .. }   => { joystick.j1_left(true); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_RIGHT), .. }  => { joystick.j1_right(true); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_FIRE), .. }   => { joystick.j1_fire(true); }
            event::Event::KeyUp { keycode: Some(UserInput::KEY_RESET), .. }  => { joystick.reset(true); }

            _ => {return true}
        }

        true
    }
}
