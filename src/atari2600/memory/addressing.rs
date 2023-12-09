use super::super::clocks;
use super::super::cpu::pc_state;
use super::memory;

pub trait Address16 {
    fn address16(&self, clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16;
    fn get_addressing_size(&self) -> u8;
    fn get_addressing_time(&self) -> u8;
}

pub struct Addressing {
    size:u8,
    cycles:u8,
}

impl Addressing {
    pub const fn new(size:u8, cycles:u8) -> Self {
        Self {
            size: size,
            cycles: cycles,
        }
    }
}

pub struct AddressingIMM {
    addressing:Addressing,
}

impl AddressingIMM {
    pub const fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 0),
        }
    }

    pub fn address(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, _: &memory::Memory) -> u16 {
        pc_state.get_pc() + 1
    }
}

pub struct AddressingZP {
    addressing:Addressing,
}

impl AddressingZP {
    pub const fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 1),
        }
    }

    pub fn address(&self, clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        memory.read(clock, pc_state.get_pc().wrapping_add(1)) as u16
    }
}

pub struct AddressingIZY {
    addressing:Addressing,
}

impl AddressingIZY {
    pub const fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 3),
        }
    }

    pub fn address(&self, clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        let tmp8 = memory.read(clock, pc_state.get_pc().wrapping_add(1));
                
        memory.read16(clock, tmp8 as u16).wrapping_add(pc_state.get_y() as u16)
    }
}

pub struct AddressingIZX {
    addressing:Addressing,
}

impl AddressingIZX {
    pub const fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 4),
        }
    }

    pub fn address(&self, clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        let tmp8 = memory.read(clock, pc_state.get_pc().wrapping_add(1)).wrapping_add(pc_state.get_x());
        memory.read16(clock, tmp8 as u16)
    }
}

pub struct AddressingZPX {
    addressing:Addressing,
}

impl AddressingZPX {
    pub const fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 2),
        }
    }

    pub fn address(&self, clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        (memory.read(clock, pc_state.get_pc().wrapping_add(1)).wrapping_add(pc_state.get_x())) as u16
    }
}

pub struct AddressingZPY {
    addressing:Addressing,
}

impl AddressingZPY {
    pub const fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 2),
        }
    }

    pub fn address(&self, clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        (memory.read(clock, pc_state.get_pc().wrapping_add(1)).wrapping_add(pc_state.get_y())) as u16
    }
}

pub struct AllAddressingModes {
}

impl AllAddressingModes {
    pub fn address_zpy(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        (memory.read(clock, pc_state.get_pc().wrapping_add(1)).wrapping_add(pc_state.get_y().wrapping_add(1))) as u16
    }

    pub fn address_abs(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        memory.read16(clock, pc_state.get_pc().wrapping_add(1))
    }

    pub fn address_indirect(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        let indirect_addr = memory.read16(clock, pc_state.get_pc().wrapping_add(1));
        memory.read16(clock, indirect_addr)
    }

    pub fn address_aby(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        let address_tmp = memory.read16(clock, pc_state.get_pc().wrapping_add(1));
        let tmp16:u16 = address_tmp + pc_state.get_y() as u16;

        // TODO: Add page delays
//        if (check_page_delay):
//            self._last_page_delay = self.has_page_clock_delay(address_tmp, tmp16)

        return tmp16
    }

    pub fn address_abx(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        let address_tmp = memory.read16(clock, pc_state.get_pc().wrapping_add(1));
        let tmp16:u16 = address_tmp + pc_state.get_x() as u16;

        // TODO: Add page delays
//        if (check_page_delay):
//            self._last_page_delay = self.has_page_clock_delay(address_tmp, tmp16)

        return tmp16
    }

    pub fn address_accumulator(clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        // TODO: Check implementation.
        0
    }

}

macro_rules! impl_addressing_struct {
     ($type:ident, $size:expr, $cycles:expr, $fn_name:tt)  => {
        pub struct $type {
            addressing:Addressing,
        }
        
        impl $type {
            pub const fn new() -> Self {
                Self {
                    addressing:Addressing::new($size, $cycles),
                }
            }
        
            pub fn address(&self, clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
                AllAddressingModes::$fn_name(clock, pc_state, memory)
            }
        }
    };
}

// Create the different structures and map them to their respective adressing functions
impl_addressing_struct!(AddressingAbs, 2, 2, address_abs);
impl_addressing_struct!(AddressingIndirect, 2, 4, address_indirect);
impl_addressing_struct!(AddressingAby, 2, 2, address_aby); // TODO: Additional time?
impl_addressing_struct!(AddressingAbx, 2, 2, address_abx); // TODO: Additional time?
impl_addressing_struct!(AddressingAccumulator, 0, 0, address_accumulator);

// Common functions associated with 'addressing' types.
// TODO: See if there's a less macro way
macro_rules! impl_addressing {
     ($type:ty)  => {
            impl Address16 for $type {
               fn address16(&self, clock: &clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
                   self.address(clock, pc_state, memory)
               }

               fn get_addressing_size(&self) -> u8 { self.addressing.size }
               fn get_addressing_time(&self) -> u8 { self.addressing.cycles  * pc_state::PcState::CYCLES_TO_CLOCK}
               }
    };
}

impl_addressing!(AddressingIZY);
impl_addressing!(AddressingIMM);
impl_addressing!(AddressingIZX);

impl_addressing!(AddressingZP);
impl_addressing!(AddressingZPX);
impl_addressing!(AddressingZPY);

impl_addressing!(AddressingAbs);
impl_addressing!(AddressingIndirect);
impl_addressing!(AddressingAby);
impl_addressing!(AddressingAbx);
impl_addressing!(AddressingAccumulator);

pub trait ReadData {
    fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8;
    fn get_reading_time(&self) -> u8;
}

pub struct MemoryRead {
    cycles:u8,
}

impl MemoryRead {
    pub const fn new() -> Self {
        Self {cycles: 2}
    }

    fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
        memory.read(clock, address)
    }
}

pub struct AccumulatorRead {
    cycles:u8,
}

impl AccumulatorRead {
    pub const fn new() -> Self {
        Self {cycles: 2}
    }

    fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
        pc_state.get_a()
    }
}

pub struct NullRead {
    cycles:u8,
}

impl NullRead {
    pub const fn new() -> Self {
        Self {cycles: 1}
    }

    fn read(&self, clock: &clocks::Clock, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
        0
    }
}


macro_rules! impl_read_data {
    ($type:ty)  => {
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

pub struct MemoryWrite { cycles:u8}

impl MemoryWrite {
    pub const fn new() -> Self {
        Self {cycles:2}
    }

    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
        memory.write(clock, address, data);
    }
}

pub struct AccumulatorWrite { cycles:u8}

impl AccumulatorWrite {
    pub const fn new() -> Self {
        Self {cycles:0}
    }

    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
        pc_state.set_a(data);
    }
}

pub struct RegisterWrite { cycles:u8}

impl RegisterWrite {
    pub const fn new() -> Self {
        Self {cycles:1}
    }

    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
        memory.write(clock, address, data);
    }
}

macro_rules! impl_write_data {
    ($type:ty)  => {
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
    fn write(&self, clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) { }
    fn get_writing_time(&self) -> u8 { 0 }
}

