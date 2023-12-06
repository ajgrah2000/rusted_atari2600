use super::pc_state;
use super::super::clocks;
use super::super::ports;
use super::super::graphics;
use super::super::memory::memory;
use super::instructions;
use std::time;

pub struct Core {
    pub clock: clocks::Clock,
    pub memory: memory::Memory,
    pc_state: pc_state::PcState,
    pub ports: ports::Ports,
    raw_display: Vec<u8>,
    start_time: time::SystemTime,
}

impl Core {
    const PROGRAM_ENTRY_ADDR:u16 = 0xFFFC;

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

    pub fn reset(&mut self) {
        // Initialise the PC state with the program entry point.
        self.pc_state.set_pc(self.memory.read16(&self.clock, Core::PROGRAM_ENTRY_ADDR));
    }

    pub fn step(&mut self, debug: bool, realtime:bool) {

        let op_code = self.memory.read(&self.clock, self.pc_state.get_pc());

        if debug {
            print!(
                "cycles:{} 0x{:X} {:X} (0x{:X}) ",
                (self.clock.ticks.wrapping_sub(self.memory.stella.debug_clock()))/pc_state::PcState::CYCLES_TO_CLOCK as u64,
                op_code,
                self.pc_state.get_pc(),
                self.memory.read(&self.clock, self.pc_state.get_pc().wrapping_add(1))
            );
            println!("{}", self.pc_state);
        }

        instructions::Instruction::execute(
            op_code,
            &mut self.clock,
            &mut self.memory,
            &mut self.pc_state,
            &mut self.ports,
        );

    }

    pub fn export(&mut self) -> bool {
        // TODO
        // Add trigger for re-drawing stella graphics
        false
    }

    pub fn generate_display(&mut self, buffer: &mut [u8]) {
        // Function to populate the display buffer drawn to the 2D texture/canvas/window.
        buffer.clone_from_slice(self.raw_display.as_slice());
    }
}

