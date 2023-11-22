use super::super::cpu::pc_state;
use super::memory;

pub trait Address16 {
    fn address16(&self, pc_state: &pc_state::PcState, memory: &memory::Memory) -> u16;
    fn get_addressing_size(&self) -> u8;
    fn get_timing(&self) -> u8;
}

pub struct Addressing {
    size:u8,
    time:u8,
}

impl Addressing {
    pub fn new(size:u8, time:u8) -> Self {
        Self {
            size: size,
            time: time * pc_state::PcState::CYCLES_TO_CLOCK,
        }
    }
}

pub struct AddressingIMM {
    addressing:Addressing,
}

impl AddressingIMM {
    pub fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 0),
        }
    }

    pub fn address(pc_state: &pc_state::PcState) -> u16 {
        pc_state.get_pc() + 1
    }
}

impl Address16 for AddressingIMM {
    fn address16(&self, pc_state: &pc_state::PcState, memory: &memory::Memory) -> u16 {
        AddressingIMM::address(pc_state)
    }

    fn get_addressing_size(&self) -> u8 { self.addressing.size }
    fn get_timing(&self) -> u8 { self.addressing.time }
}

pub trait ReadData {
    fn read(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8;
    fn get_reading_time(&self) -> u8;
}

pub struct MemoryRead {}

impl MemoryRead {
    pub fn new() -> Self {
        Self {}
    }
}

impl ReadData for MemoryRead {
    fn read(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
        memory.read(address)
    }

    fn get_reading_time(&self) -> u8 {
        2 * pc_state::PcState::CYCLES_TO_CLOCK
    }
}

pub trait WriteData {
    fn write(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8);
    fn get_writing_time(&self) -> u8;
}

pub struct MemoryWrite {}

impl MemoryWrite {
    pub fn new() -> Self {
        Self {}
    }
}

impl WriteData for MemoryWrite {
    fn write(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
        memory.write(address, data);
    }

    fn get_writing_time(&self) -> u8 {
        2 * pc_state::PcState::CYCLES_TO_CLOCK
    }
}

// TODO: Fix Null write
pub struct MemoryNull {}

impl MemoryNull {
    pub fn new() -> Self {
        Self {}
    }
}

impl WriteData for MemoryNull {
    fn write(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) { }
    fn get_writing_time(&self) -> u8 { 0 }
}



