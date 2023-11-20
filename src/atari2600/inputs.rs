use sdl2::keyboard; // Keycode
use sdl2::event; // Keycode

#[derive(Clone,Copy)]
pub struct Joystick {
}

impl Joystick {
    pub fn new() -> Self {
        Self {
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

pub struct Input {
}

impl Input {
    const KEY_UP:keyboard::Keycode     = keyboard::Keycode::Up;
    const KEY_DOWN:keyboard::Keycode   = keyboard::Keycode::Down;
    const KEY_LEFT:keyboard::Keycode   = keyboard::Keycode::Left;
    const KEY_RIGHT:keyboard::Keycode  = keyboard::Keycode::Right;
    const KEY_FIRE:keyboard::Keycode   = keyboard::Keycode::Z;
    const KEY_RESET:keyboard::Keycode  = keyboard::Keycode::R;
    const KEY_QUIT:keyboard::Keycode   = keyboard::Keycode::Escape;

    pub fn print_keys() {
        println!("Key mappings (Joystick 1):");
        println!("Up: {}, Down: {}, Left: {}, Right: {}", Input::KEY_UP, Input::KEY_DOWN, Input::KEY_LEFT, Input::KEY_RIGHT);
        println!("Fire: {}", Input::KEY_FIRE);
        println!("Reset: {}", Input::KEY_RESET);
        println!();
        println!("Quit: {}", Input::KEY_QUIT);
    }

    // Return 'true' if handled, otherwise 'false' (ie quit)
    pub fn handle_events(event:event::Event, joystick:&mut Joystick) -> bool {
        match event {
            event::Event::Quit { .. } | 
            event::Event::KeyDown { keycode: Some(Input::KEY_QUIT), ..  } => { return false }

            event::Event::KeyDown { keycode: Some(Input::KEY_UP), .. }     => { joystick.j1_up(false); }
            event::Event::KeyDown { keycode: Some(Input::KEY_DOWN), .. }   => { joystick.j1_down(false); }
            event::Event::KeyDown { keycode: Some(Input::KEY_LEFT), .. }   => { joystick.j1_left(false); }
            event::Event::KeyDown { keycode: Some(Input::KEY_RIGHT), .. }  => { joystick.j1_right(false); }
            event::Event::KeyDown { keycode: Some(Input::KEY_FIRE), .. }   => { joystick.j1_fire(false); }
            event::Event::KeyDown { keycode: Some(Input::KEY_RESET), .. }  => { joystick.reset(false); }

            event::Event::KeyUp { keycode: Some(Input::KEY_UP), .. }     => { joystick.j1_up(true); }
            event::Event::KeyUp { keycode: Some(Input::KEY_DOWN), .. }   => { joystick.j1_down(true); }
            event::Event::KeyUp { keycode: Some(Input::KEY_LEFT), .. }   => { joystick.j1_left(true); }
            event::Event::KeyUp { keycode: Some(Input::KEY_RIGHT), .. }  => { joystick.j1_right(true); }
            event::Event::KeyUp { keycode: Some(Input::KEY_FIRE), .. }   => { joystick.j1_fire(true); }
            event::Event::KeyUp { keycode: Some(Input::KEY_RESET), .. }  => { joystick.reset(true); }

            _ => {return true}
        }

        true
    }
}
