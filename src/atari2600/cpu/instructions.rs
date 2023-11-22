use super::super::clocks;
use super::super::memory::memory;
use super::super::memory::addressing;
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

            0xA2 => { instruction_set::ldx(clock, pc_state, memory, addressing::AddressingIMM::new(), addressing::MemoryRead::new()); }

            0xA9 => { instruction_set::lda(clock, pc_state, memory, addressing::AddressingIMM::new(), addressing::MemoryRead::new()); }

            0x18 => { instruction_set::clc(clock, pc_state); }
            0xD8 => { instruction_set::cld(clock, pc_state); }
            0x58 => { instruction_set::cli(clock, pc_state); }
            0xB8 => { instruction_set::clv(clock, pc_state); }

            0x38 => { instruction_set::sec(clock, pc_state); }
            0x78 => { instruction_set::sei(clock, pc_state); }
            0xF8 => { instruction_set::sed(clock, pc_state); }
            _ => {
                panic!("Opcode not implemented: 0x{:x}", op_code);
            }
        }
    }
}
