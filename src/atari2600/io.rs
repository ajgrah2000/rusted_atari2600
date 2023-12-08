use super::clocks;
use super::inputs;

pub trait ReadWriteMemory {
    fn read(&mut self, clock: &clocks::Clock, address:u16) -> u8;
    fn write(&mut self, clock: &mut clocks::Clock, address:u16, data:u8);
}

pub trait DebugClock {
    fn debug_clock(&mut self) -> clocks::ClockType;
}

pub trait StellaIO: ReadWriteMemory + DebugClock {
    fn export(&mut self) -> bool;
    fn generate_display(&mut self, buffer: &mut [u8]);
    fn set_inputs(&mut self, inputs: inputs::Input);
}

pub trait RiotIO: ReadWriteMemory {
    fn set_inputs(&mut self, inputs: inputs::Input);
}
