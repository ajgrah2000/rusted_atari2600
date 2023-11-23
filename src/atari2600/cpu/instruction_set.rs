use super::super::clocks;
use super::pc_state;
use super::super::memory::addressing;
use super::super::memory::memory;

//TODO: Do actual instructions
pub fn noop(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    clock.increment(4);
}

pub fn single_byte_instruction <R, W, I: Fn(&mut clocks::Clock, &mut pc_state::PcState, u8) -> u8 > (clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, read:R, write: W, instruction: I) where
R: pc_state::ReadReg8, W: pc_state::WriteReg8 {
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);

    let data = read.get(pc_state);
    let result = instruction(clock, pc_state, data);
    write.set(pc_state, result);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);

    pc_state.increment_pc(1);
}

pub fn read_write_instruction <A, R, W, I: Fn(&mut clocks::Clock, &mut pc_state::PcState, &mut memory::Memory, u8) -> u8 >
        (clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: &A, read: R, write: W, instruction: I) 
    where A: addressing::Address16, 
        R: addressing::ReadData, 
        W: addressing::WriteData 
{
    let addr = address.address16(pc_state, memory);
    let mut execute_time = address.get_addressing_time();

    let value = read.read(pc_state, memory, addr);
    execute_time += read.get_reading_time();

    execute_time += write.get_writing_time();

    let data = instruction(clock, pc_state, memory, value);

    clock.increment(execute_time as u32);

    write.write(pc_state, memory ,addr, data);

    pc_state.increment_pc((address.get_addressing_size() + 1) as i8);
}

pub fn ldx (clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.set_x(data);
    pc_state::set_status_nz(pc_state, data);
    0
}

pub fn lda (clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.set_a(data);
    pc_state::set_status_nz(pc_state, data);
    0
}

pub fn sta (clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.get_a()
}

pub fn sty (clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.get_y()
}

pub fn stx (clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.get_x()
}

pub fn sax (clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.get_a() & pc_state.get_x()
}

pub fn clc(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_c(false);
    0
}

pub fn cld(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_d(false);
    0
}

pub fn cli(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_i(false);
    0
}

pub fn clv(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_v(false);
    0
}

pub fn sec(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_c(true);
    0
}

pub fn sei(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_i(true);
    0
}

pub fn sed(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_d(true);
    0
}


