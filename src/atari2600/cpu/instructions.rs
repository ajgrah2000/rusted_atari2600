use super::super::clocks;
use super::super::memory::memory;
use super::super::ports;
use super::pc_state;
use super::instruction_set;

pub struct Instruction {}

impl Instruction {
    pub fn execute(
        op_code: u8,
        clock: &mut clocks::Clock,
        memory: &mut memory::Memory,
        pc_state: &mut pc_state::PcState,
        ports: &mut ports::Ports) {
        match op_code {
            0x00 => {
                // TODO: put the actual instruction for '0x00'  here
                instruction_set::noop(clock, pc_state);
            }
            _ => {
                panic!("Opcode not implemented: {:x}", op_code);
            }
        }
    }
}
