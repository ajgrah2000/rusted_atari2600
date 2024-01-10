use super::super::clocks;
use super::super::cpu::pc_state;
use super::memory;

pub trait Address16 {
    fn address16(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16;
    fn get_addressing_size() -> u8;
    fn get_addressing_time() -> u8;
}

fn did_index_cross_page(base_address: u16, result_address: u16) -> bool {
    (base_address & 0xFF00) != (result_address & 0xFF00)
}

pub struct AllAddressingModes {}

impl AllAddressingModes {
    // A 'page cross', is when there was a carry during 'indexed addressing'
    // If the initial address read was on the same page, then the 'pipepline' read will be valid.
    // If the carry was needed, then a 're-read' of the 'correct' address is needed, adding an extra delay.
    // The 'delay' is only applicable to op-codes that immediately use the 'result address' (ie LDA).  Generally, for functions that use the address to 'write',
    // It appears as though there's generally time to 'fix' the address before it's needed for the 'write' operation (eg STA).

    pub fn address_accumulator(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        // TODO: Check implementation.
        0
    }
    pub fn address_imm(clock: &clocks::Clock, pc_state: &pc_state::PcState, _: &memory::Memory, page_delay: bool) -> u16 {
        pc_state.get_pc().wrapping_add(1)
    }

    pub fn address_zp(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        memory.read(clock, pc_state.get_pc().wrapping_add(1)) as u16
    }

    pub fn address_izx(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        let tmp8 = memory.read(clock, pc_state.get_pc().wrapping_add(1)).wrapping_add(pc_state.get_x());
        memory.read16(clock, tmp8 as u16)
    }

    pub fn address_zpx(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        (memory.read(clock, pc_state.get_pc().wrapping_add(1)).wrapping_add(pc_state.get_x())) as u16
    }

    pub fn address_zpy(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        (memory.read(clock, pc_state.get_pc().wrapping_add(1)).wrapping_add(pc_state.get_y())) as u16
    }

    pub fn address_abs(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        memory.read16(clock, pc_state.get_pc().wrapping_add(1))
    }

    pub fn address_indirect(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        let indirect_addr = memory.read16(clock, pc_state.get_pc().wrapping_add(1));
        memory.read16(clock, indirect_addr)
    }

    pub fn address_izy(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        let tmp8 = memory.read(clock, pc_state.get_pc().wrapping_add(1));
        let address_tmp = memory.read16(clock, tmp8 as u16);
        let tmp16 = address_tmp.wrapping_add(pc_state.get_y() as u16);

        if page_delay && did_index_cross_page(address_tmp, tmp16) {
            clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
        }

        tmp16
    }

    pub fn address_aby(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        let address_tmp = memory.read16(clock, pc_state.get_pc().wrapping_add(1));
        let tmp16: u16 = address_tmp + pc_state.get_y() as u16;

        if page_delay && did_index_cross_page(address_tmp, tmp16) {
            clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
        }

        tmp16
    }

    pub fn address_abx(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, page_delay: bool) -> u16 {
        let address_tmp = memory.read16(clock, pc_state.get_pc().wrapping_add(1));
        let tmp16: u16 = address_tmp + pc_state.get_x() as u16;

        if page_delay && did_index_cross_page(address_tmp, tmp16) {
            clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
        }

        tmp16
    }
}

pub enum Addressing {
    Imm, Zp, Izy, IZYPageDelay, Izx, Zpx, Zpy,
    Abs, Indirect, Aby, Abx, Accumulator,
    AbyPageDelay, AbxPageDelay,
}

impl Addressing {
    pub fn address16(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
    
        match self {
            Addressing::Imm => AllAddressingModes::address_imm(clock, pc_state, memory, false),
            Addressing::Zp => AllAddressingModes::address_zp(clock, pc_state, memory, false),
            Addressing::Izy => AllAddressingModes::address_izy(clock, pc_state, memory, false),
            Addressing::IZYPageDelay => AllAddressingModes::address_izy(clock, pc_state, memory, true),
            Addressing::Izx => AllAddressingModes::address_izx(clock, pc_state, memory, false),
            Addressing::Zpx => AllAddressingModes::address_zpx(clock, pc_state, memory, false),
            Addressing::Zpy => AllAddressingModes::address_zpy(clock, pc_state, memory, false),
            
            Addressing::Abs => AllAddressingModes::address_abs(clock, pc_state, memory, false),
            Addressing::Indirect=> AllAddressingModes::address_indirect(clock, pc_state, memory, false),
            Addressing::Aby => AllAddressingModes::address_aby(clock, pc_state, memory, false),
            Addressing::Abx => AllAddressingModes::address_abx(clock, pc_state, memory, false),
            Addressing::Accumulator => AllAddressingModes::address_accumulator(clock, pc_state, memory, false),
            
            Addressing::AbyPageDelay => AllAddressingModes::address_aby(clock, pc_state, memory, true),
            Addressing::AbxPageDelay => AllAddressingModes::address_abx(clock, pc_state, memory, true),
        }
    }

    pub fn get_addressing_size(&self) -> u8 {
        match self {
            Addressing::Imm => 1,
            Addressing::Zp => 1,
            Addressing::Izy => 1,
            Addressing::IZYPageDelay => 1,
            Addressing::Izx => 1,
            Addressing::Zpx => 1,
            Addressing::Zpy => 1,

            Addressing::Abs => 2,
            Addressing::Indirect => 2,
            Addressing::Aby => 2,
            Addressing::Abx => 2,
            Addressing::Accumulator => 0,

            Addressing::AbyPageDelay => 2,
            Addressing::AbxPageDelay => 2,
        }
    }

    pub fn get_addressing_time(&self) -> u8 {
         pc_state::PcState::CYCLES_TO_CLOCK * match self {
            Addressing::Imm => 0,
            Addressing::Zp => 1,
            Addressing::Izy => 3,
            Addressing::IZYPageDelay => 3,
            Addressing::Izx => 4,
            Addressing::Zpx => 2,
            Addressing::Zpy => 2,
            
            Addressing::Abs => 2,
            Addressing::Indirect => 4,
            Addressing::Aby => 2,
            Addressing::Abx => 2,
            Addressing::Accumulator => 0,
            
            Addressing::AbyPageDelay => 2,
            Addressing::AbxPageDelay => 2,
        }
    }
}


pub trait ReadData {
    fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8;
    fn get_reading_time(&self) -> u8;
}

pub struct MemoryRead {
    cycles: u8,
}

impl MemoryRead {
    pub const fn new() -> Self {
        Self { cycles: 2 }
    }

    fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
        memory.read(clock, address)
    }
}

pub struct AccumulatorRead {
    cycles: u8,
}

impl AccumulatorRead {
    pub const fn new() -> Self {
        Self { cycles: 2 }
    }

    fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
        pc_state.get_a()
    }
}

pub struct NullRead {
    cycles: u8,
}

impl NullRead {
    pub const fn new() -> Self {
        Self { cycles: 1 }
    }

    fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
        0
    }
}

macro_rules! impl_read_data {
    ($type:ty) => {
        impl ReadData for $type {
            fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
                self.read(clock, pc_state, memory, address)
            }

            fn get_reading_time(&self) -> u8 {
                self.cycles * pc_state::PcState::CYCLES_TO_CLOCK
            }
        }
    };
}

impl_read_data!(AccumulatorRead);
impl_read_data!(MemoryRead);
impl_read_data!(NullRead);

pub trait WriteData {
    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8);
    fn get_writing_time(&self) -> u8;
}

pub struct MemoryWrite {
    cycles: u8,
}

impl MemoryWrite {
    pub const fn new() -> Self {
        Self { cycles: 2 }
    }

    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
        memory.write(clock, address, data);
    }
}

pub struct AccumulatorWrite {
    cycles: u8,
}

impl AccumulatorWrite {
    pub const fn new() -> Self {
        Self { cycles: 0 }
    }

    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
        pc_state.set_a(data);
    }
}

pub struct RegisterWrite {
    cycles: u8,
}

impl RegisterWrite {
    pub const fn new() -> Self {
        Self { cycles: 1 }
    }

    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
        memory.write(clock, address, data);
    }
}

macro_rules! impl_write_data {
    ($type:ty) => {
        impl WriteData for $type {
            fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
                self.write(clock, pc_state, memory, address, data);
            }

            fn get_writing_time(&self) -> u8 {
                self.cycles * pc_state::PcState::CYCLES_TO_CLOCK
            }
        }
    };
}

impl_write_data!(AccumulatorWrite);
impl_write_data!(MemoryWrite);
impl_write_data!(RegisterWrite);

// TODO: Fix Null write
pub struct MemoryNull {}

impl MemoryNull {
    pub const fn new() -> Self {
        Self {}
    }
}

impl WriteData for MemoryNull {
    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {}
    fn get_writing_time(&self) -> u8 {
        0
    }
}
