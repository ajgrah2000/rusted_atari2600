use super::super::clocks;
use super::pc_state;
use super::super::memory::addressing;
use super::super::memory::memory;

//TODO: Do actual instructions
pub fn noop(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    clock.increment(4);
}

pub fn single_byte_instruction <R, W, I: Fn(&mut clocks::Clock, &mut pc_state::PcState, u8) -> u8 >(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, read:R, write: W, instruction: I) where
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
    let addr = address.address16(clock, pc_state, memory);
    let mut execute_time = address.get_addressing_time();

    let value = read.read(clock, pc_state, memory, addr);
    execute_time += read.get_reading_time();

    execute_time += write.get_writing_time();

    let data = instruction(clock, pc_state, memory, value);

    clock.increment(execute_time as u32);

    write.write(clock, pc_state, memory ,addr, data);

    pc_state.increment_pc((address.get_addressing_size() + 1) as i16);
}

pub fn jump_sub_routine_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) 
{
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);
    
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    let adl = memory.read(clock, pc_state.get_pc());
    
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    
    // Increment before store, to catch low to high carry.
    pc_state.increment_pc(1);
    memory.write(clock, pc_state.get_s() as u16, pc_state.get_pch());
    pc_state.increment_s(-1);
    
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.write(clock, pc_state.get_s() as u16, pc_state.get_pcl());
    pc_state.increment_s(-1);
    
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    let adh = memory.read(clock, pc_state.get_pc());
    
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.set_pc(adl as u16 + ((adh as u16) << 8));
}

pub fn branch_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, condition_mask: u8, condition: u8) 
{
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);

    if (pc_state.get_p() & condition_mask) == condition {
//        tmp16 = self.pc_state.PC
        let delta = memory.read(clock, pc_state.get_pc().wrapping_add(1));
        if delta & 0x80 != 0 {
//            pc_state.increment_pc(delta - 0x100);
            pc_state.increment_pc((delta as i8) as i16);
        } else {
            pc_state.increment_pc(delta as i16);
        }
//        # If branch to same page add 1, else add 2
//        # TODO: Confirm if starting address is 'tmp16' or 'tmp16+2'
// TODO: page clock delay.
//        self.page_clocks_delay(tmp16+2, self.pc_state.PC+2)

        clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    }
    
    pc_state.increment_pc(2);
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
}

pub fn asl(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_c(0 != (data >> 7) & 0x1);
    let left_shift =  (data << 1) as u8;
    pc_state::set_status_nz(pc_state, left_shift);
    left_shift
}

pub fn lsr(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state.set_flag_c(0 != data & 0x1);
    let right_shift =  data >> 1;
    pc_state::set_status_nz(pc_state, right_shift);
    right_shift
}

pub fn ldx(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.set_x(data);
    pc_state::set_status_nz(pc_state, data);
    0
}

pub fn lda(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.set_a(data);
    pc_state::set_status_nz(pc_state, data);
    0
}

pub fn sta(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.get_a()
}

pub fn sty(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.get_y()
}

pub fn stx(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.get_x()
}

pub fn sax(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data:u8) -> u8 {
    pc_state.get_a() & pc_state.get_x()
}

pub fn dec(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    let incremented = data.wrapping_sub(1);
    pc_state::set_status_nz(pc_state, incremented);
    incremented
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

pub fn inc(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    let incremented = data.wrapping_add(1);
    pc_state::set_status_nz(pc_state, incremented);
    incremented
}

pub fn t_no_status(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    data
}

pub fn t_status(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, data:u8) -> u8 {
    pc_state::set_status_nz(pc_state, data);
    data
}




