use super::clocks;
use super::ports;
use super::inputs;
use super::graphics;

use sdl2::pixels;

pub struct Core {
    pub ports: ports::Ports,
}

impl Core {
  pub fn new(ports: ports::Ports) -> Self {
      Self {
          ports
      }
  }
}

pub struct Atari2600 {
    core: Core,
    debug: bool,
    realtime: bool,
    stop_clock: clocks::ClockType,
    fullscreen: bool,
}

impl Atari2600 {
    pub fn build_atari2600(cartridge_name: String) -> Core {
        let ports = ports::Ports::new();
        Core::new(ports)
    }

    pub fn power_atari2600(&mut self) {
        inputs::Input::print_keys();

        let mut frame_width = graphics::stella::Constants::BLIT_WIDTH;
        // If not in full screen, default to using a bigger window.
        if !self.fullscreen {frame_width *= 1;}
        let frame_height = ((frame_width as u32) * (graphics::stella::Constants::BLIT_HEIGHT as u32) / (graphics::stella::Constants::BLIT_WIDTH as u32)) as u16;

        println!("powering on Atari 2600 Emulator.");

        let window_size = graphics::display::WindowSize::new(frame_width, frame_height, graphics::stella::Constants::BLIT_WIDTH as u16, graphics::stella::Constants::BLIT_HEIGHT as u16, self.fullscreen);

        self.main_loop(window_size, graphics::display::SDLUtility::PIXEL_FORMAT);
    }

    pub fn new(debug: bool, realtime: bool, stop_clock:clocks::ClockType, cartridge_name: String, fullscreen: bool) -> Self {
    
        let core = Self::build_atari2600(cartridge_name);
        Self { core, debug, realtime, stop_clock, fullscreen }
    }

    pub fn main_loop(&mut self, mut window_size: graphics::display::WindowSize, pixel_format: pixels::PixelFormatEnum) {
        let mut sdl_context = sdl2::init().unwrap();

        let mut canvas = graphics::display::SDLUtility::create_canvas(
            &mut sdl_context,
            "rust-atari2600 emulator",
            window_size.frame_width,
            window_size.frame_height,
            window_size.fullscreen,
        );

        canvas.set_logical_size(window_size.console_width as u32, window_size.console_height as u32).unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                if !inputs::Input::handle_events(event, &mut self.core.ports.joysticks) {
                    break 'running;
                };
            }
        }
    }
}
