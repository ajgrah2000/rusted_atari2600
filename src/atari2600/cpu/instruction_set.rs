use super::super::clocks;
use super::pc_state;
use super::super::memory::addressing;
use super::super::memory::memory;

//TODO: Do actual instructions
pub fn noop(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    clock.increment(4);
}

pub fn ldx <A, R>(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: A, read: R) where A: addressing::Address16, R: addressing::ReadData {
    let value = read.read(pc_state, memory, address.address16(pc_state, memory));
    pc_state.set_x(value);
    pc_state::set_status_nz(pc_state, value);

    pc_state.increment_pc(address.get_addressing_size() as i8);
}

pub fn lda <A, R>(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: A, read: R) where A: addressing::Address16, R: addressing::ReadData {
    let value = read.read(pc_state, memory, address.address16(pc_state, memory));
    pc_state.set_a(value);
    pc_state::set_status_nz(pc_state, value);

    pc_state.increment_pc(address.get_addressing_size() as i8);
}

pub fn clc(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    pc_state.set_flag_c(false);
}

pub fn cld(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    pc_state.set_flag_d(false);
}

pub fn cli(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    pc_state.set_flag_i(false);
}

pub fn clv(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    pc_state.set_flag_v(false);
}

pub fn sec(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    pc_state.set_flag_c(true);
}

pub fn sei(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    pc_state.set_flag_i(true);
}

pub fn sed(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    pc_state.set_flag_d(true);
}


