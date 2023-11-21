use super::pc_state;
use super::super::clocks;
use super::super::ports;
use super::super::graphics;
use super::super::memory::memory;
use std::time;

pub struct Core {
    pub clock: clocks::Clock,
    memory: memory::Memory,
    pc_state: pc_state::PcState,
    pub ports: ports::Ports,
    raw_display: Vec<u8>,
    start_time: time::SystemTime,
}

impl Core {

    pub fn new(
        clock: clocks::Clock,
        memory: memory::Memory,
        pc_state: pc_state::PcState,
        ports: ports::Ports,
    ) -> Self
    {
        Self {
            clock,
            memory,
            pc_state,
            ports,
            raw_display: vec![
                0;
                (graphics::stella::Constants::BLIT_WIDTH as usize)
                    * (graphics::stella::Constants::BLIT_HEIGHT as usize)
                    * (graphics::display::SDLUtility::bytes_per_pixel() as usize)
            ],
            start_time: time::SystemTime::now(),
        }
    }

    pub fn step(&mut self, debug: bool, realtime:bool) {

        if debug {
            print!(
                "{} {:x} ({:x}) ",
                self.clock.cycles,
                self.pc_state.get_pc(),
                self.memory.read(self.pc_state.get_pc() + 1)
            );
            println!("{}", self.pc_state);
        }
    }

    pub fn generate_display(&mut self, buffer: &mut [u8]) {
        // Function to populate the display buffer drawn to the 2D texture/canvas/window.
        buffer.clone_from_slice(self.raw_display.as_slice());
    }
}

