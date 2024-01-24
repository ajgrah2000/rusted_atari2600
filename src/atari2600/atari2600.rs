use super::audio::sound;
use super::clocks;
use super::cpu;
use super::graphics;
use super::inputs;
use super::memory;
use super::ports;

use sdl2::pixels;
use sdl2::rect;
use sdl2::render;
use sdl2::video;

pub struct Atari2600 {
    core: cpu::core::Core,
    debug: bool,
    realtime: bool,
    stop_clock: clocks::ClockType,
    fullscreen: bool,
    counter: u32,

    // These appear as 'Options' to simplify delayed initialisation.
    sdl_context: Option<sdl2::Sdl>,
    canvas: Option<render::Canvas<video::Window>>,
    audio_queue: Option<Box<dyn sound::SoundQueue>>,
}

impl Atari2600 {
    const DISPLAY_UPDATES_PER_KEY_EVENT: u32 = 10000; // Number of display updates per key press event. (reduces texture creation overhead).
    const CPU_STEPS_PER_AUDIO_UPDATE: u32 = 50; // Number of times to step the CPU before updating the audio.

    pub fn build_atari2600(cartridge_name: String, cartridge_type: memory::cartridge::CartridgeType, debug: bool, realtime: bool, pal_palette: bool) -> cpu::core::Core {
        let clock = clocks::Clock::new();
        let pc_state = cpu::pc_state::PcState::new();
        // Default Cartridge.
        let mut cartridge = memory::cartridge::get_new_carterage(cartridge_name.clone(), cartridge_type);
        match cartridge.load() {
            Ok(()) => {
                println!("Ok");
            }
            Err(e) => {
                panic!("Error loading cartridge \"{}\".\n {}", cartridge_name, e);
            }
        }

        let stella = graphics::stella::Stella::new(debug, realtime, pal_palette);
        let riot = memory::riot::Riot::new();
        let memory = memory::memory::Memory::new(cartridge, Box::new(stella), Box::new(riot));

        let ports = ports::Ports::new();

        let mut core = cpu::core::Core::new(clock, memory, pc_state, ports);
        core.reset();

        core
    }

    pub fn get_console_size() -> graphics::display::ConsoleSize {
        graphics::display::ConsoleSize::new(graphics::stella::Constants::ATARI2600_WIDTH, graphics::stella::Constants::ATARI2600_HEIGHT)
    }

    pub fn get_window_size() -> graphics::display::WindowSize {
        // Default scaling (if not full screen)
        const PIXEL_WIDTH: u8 = 2;
        const PIXEL_HEIGHT: u8 = 2;

        const BLIT_WIDTH: u16 = graphics::stella::Constants::ATARI2600_WIDTH * graphics::stella::Constants::PIXEL_WIDTH_STRETCH as u16 * (PIXEL_WIDTH as u16);
        const BLIT_HEIGHT: u16 = graphics::stella::Constants::ATARI2600_HEIGHT * (PIXEL_HEIGHT as u16);

        let frame_width = BLIT_WIDTH;
        let frame_height = ((frame_width as u32) * (BLIT_HEIGHT as u32) / (BLIT_WIDTH as u32)) as u16;

        let console_size = Self::get_console_size();
        graphics::display::WindowSize::new(frame_width, frame_height, console_size, false)
    }

    pub fn run_atari2600(me: &mut Atari2600) -> bool {
        let console_size = Self::get_console_size();

        let pixel_format = graphics::display::SDLUtility::PIXEL_FORMAT;
        let mut event_pump = me.sdl_context.as_mut().expect("Should be here").event_pump().unwrap();
        for event in event_pump.poll_iter() {
            graphics::display::SDLUtility::handle_events(&event);

            if !inputs::UserInput::handle_events(event, &mut me.core.ports.joysticks) {
                return false;
            };
            me.core.memory.riot.set_inputs(me.core.ports.joysticks.input);
            me.core.memory.stella.set_inputs(me.core.ports.joysticks.input);
        }

        // Need to temporarily move the mutable fields out of 'self' to avoid multiple borrows of mutable self.
        if !me.draw_loop(pixel_format, &console_size, Atari2600::DISPLAY_UPDATES_PER_KEY_EVENT) {
            return false;
        }
        true
    }

    pub fn power_atari2600(&mut self) {
        inputs::UserInput::print_keys();

        let window_size = Self::get_window_size();

        self.configure_sdl(window_size, graphics::display::SDLUtility::PIXEL_FORMAT);
    }

    pub fn new(debug: bool, realtime: bool, stop_clock: clocks::ClockType, cartridge_name: String, cartridge_type: memory::cartridge::CartridgeType, fullscreen: bool, pal_palette: bool) -> Self {
        let core = Self::build_atari2600(cartridge_name, cartridge_type, debug, realtime, pal_palette);
        Self { core, debug, realtime, stop_clock, fullscreen, counter:0, sdl_context:None , canvas:None, audio_queue:None}
    }

    pub fn draw_loop(&mut self,  pixel_format: pixels::PixelFormatEnum, console_size: &graphics::display::ConsoleSize, iterations: u32) -> bool {
        // Number of iterations to do before getting a new texture.
        // These loops will update the display, but currently events aren't checked in this time.

        let audio_queue = self.audio_queue.as_mut().expect("Optional audio not set");
        let canvas = self.canvas.as_mut().expect("Optional canvas not set");

        // Creating the texture creator and texture is slow, so perform multiple display updates per creation.
        let texture_creator = graphics::display::SDLUtility::texture_creator(canvas);
        let mut texture = graphics::display::SDLUtility::create_texture(&texture_creator, pixel_format, console_size.console_width, console_size.console_height);

        let mut audio_steps = 0;
        let mut display_refreshes = 0;
        while display_refreshes < iterations {
            if self.stop_clock > 0 && self.core.clock.ticks > self.stop_clock {
                return false;
            }
            self.core.step(self.debug, self.realtime);
            self.core.memory.stella.step_tia_sound(&self.core.clock);

            if 0 == audio_steps % Atari2600::CPU_STEPS_PER_AUDIO_UPDATE {
                // Top-up the audio queue
                // TODO: Change this thing of beauty to something even better. 
                sound::SDLUtility::top_up_audio_queue(&mut **audio_queue, |fill_size| self.core.memory.stella.get_next_audio_chunk(fill_size));
            }
            audio_steps += 1;

            if self.core.memory.stella.export() {
                texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| self.core.memory.stella.generate_display(buffer)).unwrap();

                canvas.clear();
                canvas
                    .copy(
                        &texture,
                        None,
                        Some(rect::Rect::new(0, 0, graphics::stella::Constants::PIXEL_WIDTH_STRETCH as u32 * console_size.console_width as u32, console_size.console_height as u32)),
                    )
                    .unwrap();
                canvas.present();
            }
            display_refreshes += 1;
        }
        true
    }

    pub fn configure_sdl(&mut self, window_size: graphics::display::WindowSize, pixel_format: pixels::PixelFormatEnum) {

        let mut sdl_context = sdl2::init().unwrap();

        let mut canvas = graphics::display::SDLUtility::create_canvas(&mut sdl_context, "rust-atari2600 emulator", window_size.frame_width, window_size.frame_height, window_size.fullscreen);

        canvas.set_logical_size(graphics::stella::Constants::PIXEL_WIDTH_STRETCH as u32 * window_size.console_size.console_width as u32, window_size.console_size.console_height as u32).unwrap();

        let audio_queue = sound::SDLUtility::get_audio_queue(&mut sdl_context);

        // Set members once update/modifications have been done.
        self.sdl_context = Some(sdl_context);
        self.canvas = Some(canvas);
        self.audio_queue = Some(audio_queue);
    }
}

impl Drop for Atari2600 {
    fn drop(&mut self) {
        println!("Atari2600 is being dropped");
    }
}
