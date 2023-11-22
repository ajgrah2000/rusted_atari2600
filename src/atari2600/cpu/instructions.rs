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

            0xA2 => { instruction_set::read_write_instruction(clock, pc_state, memory, &addressing::AddressingIMM::new(), addressing::MemoryRead::new(), addressing::MemoryNull::new(), instruction_set::ldx); }

            0xA9 => { instruction_set::read_write_instruction(clock, pc_state, memory, &addressing::AddressingIMM::new(), addressing::MemoryRead::new(), addressing::MemoryNull::new(), instruction_set::lda); }

            0x18 => { instruction_set::single_byte_instruction(clock, pc_state, instruction_set::clc); }
            0xD8 => { instruction_set::single_byte_instruction(clock, pc_state, instruction_set::cld); }
            0x58 => { instruction_set::single_byte_instruction(clock, pc_state, instruction_set::cli); }
            0xB8 => { instruction_set::single_byte_instruction(clock, pc_state, instruction_set::clv); }

            0x38 => { instruction_set::single_byte_instruction(clock, pc_state, instruction_set::sec); }
            0x78 => { instruction_set::single_byte_instruction(clock, pc_state, instruction_set::sei); }
            0xF8 => { instruction_set::single_byte_instruction(clock, pc_state, instruction_set::sed); }
            _ => {
                panic!("Opcode not implemented: 0x{:x}", op_code);
            }
        }
    }
}
