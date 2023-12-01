use super::super::clocks;
use super::super::memory::memory;
use super::super::memory::addressing;
use super::super::ports;
use super::pc_state;
use super::instruction_set;

pub struct Instruction {}

// There's likely a better way to specify the memory types, but this achieves the intent.
const ADDR_IMM:addressing::AddressingIMM = addressing::AddressingIMM::new();
const ADDR_ZP:addressing::AddressingZP = addressing::AddressingZP::new();
const ADDR_ZPX:addressing::AddressingZPX = addressing::AddressingZPX::new();
const ADDR_ZPY:addressing::AddressingZPY = addressing::AddressingZPY::new();
const ADDR_IZX:addressing::AddressingIZX = addressing::AddressingIZX::new();
const ADDR_IZY:addressing::AddressingIZY = addressing::AddressingIZY::new();

const ADDR_ABS:addressing::AddressingAbs = addressing::AddressingAbs::new();
const ADDR_INDIRECT:addressing::AddressingIndirect = addressing::AddressingIndirect::new();
const ADDR_ABY:addressing::AddressingAby = addressing::AddressingAby::new();
const ADDR_ABX:addressing::AddressingAbx = addressing::AddressingAbx::new();
const ADDR_ACCUMULATOR:addressing::AddressingAccumulator = addressing::AddressingAccumulator::new();

const NULL_READ:addressing::NullRead = addressing::NullRead::new();
const MEMORY_READ:addressing::MemoryRead = addressing::MemoryRead::new();
const MEMORY_WRITE:addressing::MemoryWrite = addressing::MemoryWrite::new();
const REG_WRITE:addressing::RegisterWrite = addressing::RegisterWrite::new();
const MEMORY_NULL:addressing::MemoryNull = addressing::MemoryNull::new();

const READ_NULL: pc_state::ReadNull = pc_state::ReadNull::new();
const READ_REG_X: pc_state::ReadX = pc_state::ReadX::new();
const READ_REG_Y: pc_state::ReadY = pc_state::ReadY::new();
const READ_REG_A: pc_state::ReadA = pc_state::ReadA::new();
const READ_REG_S: pc_state::ReadS = pc_state::ReadS::new();

const WRITE_NULL: pc_state::WriteNull = pc_state::WriteNull::new();
const WRITE_REG_X: pc_state::WriteX = pc_state::WriteX::new();
const WRITE_REG_Y: pc_state::WriteY = pc_state::WriteY::new();
const WRITE_REG_A: pc_state::WriteA = pc_state::WriteA::new();
const WRITE_REG_S: pc_state::WriteS = pc_state::WriteS::new();

impl Instruction {


    pub fn execute(
        op_code: u8,
        clock: &mut clocks::Clock,
        memory: &mut memory::Memory,
        pc_state: &mut pc_state::PcState,
        ports: &mut ports::Ports) {
        match op_code {

            0xEA => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_A, instruction_set::nop); }

            0x0A => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_A, instruction_set::asl); }
            0x4A => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_A, instruction_set::lsr); }
            0xE8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_X, instruction_set::inc); }
            0xC8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_Y, WRITE_REG_Y, instruction_set::inc); }
            0xCA => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_X, instruction_set::dec); }
            0x88 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_Y, WRITE_REG_Y, instruction_set::dec); }

            0x18 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::clc); }
            0xD8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::cld); }
            0x58 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::cli); }
            0xB8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::clv); }

            0x38 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::sec); }
            0x78 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::sei); }
            0xF8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::sed); }

            // Register Transfers
            0x9A => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_S, instruction_set::t_no_status); }
            0xBA => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_S, WRITE_REG_X, instruction_set::t_no_status); }
            0x8A => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_A, instruction_set::t_status); }
            0xAA => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_X, instruction_set::t_status); }
            0xA8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_Y, instruction_set::t_status); }
            0x98 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_Y, WRITE_REG_A, instruction_set::t_status); }

            // ADC
            0x61 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, instruction_set::adc); }
            0x69 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::adc); }
            0x65 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::adc); }
            0x75 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, instruction_set::adc); }
            0x71 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, instruction_set::adc); }
            0x6D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::adc); }
            0x7D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, instruction_set::adc); }
            0x79 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, instruction_set::adc); }

            // AND
            0x21 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, instruction_set::and); }
            0x29 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::and); }
            0x25 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::and); }
            0x35 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, instruction_set::and); }
            0x31 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, instruction_set::and); }
            0x2D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::and); }
            0x3D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, instruction_set::and); }
            0x39 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, instruction_set::and); }

            // CPX
            0xE0 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::cpx); }
            0xE4 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::cpx); }
            0xEC => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::cpx); }

            // CPY
            0xC0 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::cpy); }
            0xC4 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::cpy); }
            0xCC => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::cpy); }

            // CMP
            0xC1 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, instruction_set::cmp); }
            0xC9 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::cmp); }
            0xC5 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::cmp); }
            0xD5 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, instruction_set::cmp); }
            0xD1 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, instruction_set::cmp); }
            0xCD => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::cmp); }
            0xDD => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, instruction_set::cmp); }
            0xD9 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, instruction_set::cmp); }

            // DEC
            0xC6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_WRITE, instruction_set::dec); }
            0xD6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_WRITE, instruction_set::dec); }
            0xCE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_WRITE, instruction_set::dec); }
            0xDE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_WRITE, instruction_set::dec); }

            // EOR
            0x41 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, instruction_set::eor); }
            0x49 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::eor); }
            0x45 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::eor); }
            0x55 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, instruction_set::eor); }
            0x51 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, instruction_set::eor); }
            0x4D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::eor); }
            0x5D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, instruction_set::eor); }
            0x59 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, instruction_set::eor); }

            // LDA
            0xA1 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, instruction_set::lda); }
            0xA9 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::lda); }
            0xA5 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::lda); }
            0xB5 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, instruction_set::lda); }
            0xB1 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, instruction_set::lda); }
            0xAD => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::lda); }
            0xBD => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, instruction_set::lda); }
            0xB9 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, instruction_set::lda); }

            // LDX
            0xA2 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::ldx); }
            0xA6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::ldx); }
            0xB6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPY, MEMORY_READ, MEMORY_NULL, instruction_set::ldx); }
            0xAE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::ldx); }
            0xBE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, instruction_set::ldx); }

            // LDY
            0xA0 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }
            0xA4 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }
            0xB4 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }
            0xAC => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }
            0xBC => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }

            // OR
            0x01 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, instruction_set::or); }
            0x09 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::or); }
            0x05 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::or); }
            0x15 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, instruction_set::or); }
            0x11 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, instruction_set::or); }
            0x0D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::or); }
            0x1D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, instruction_set::or); }
            0x19 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, instruction_set::or); }

            // SBC
            0xE1 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, instruction_set::sbc); }
            0xE9 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::sbc); }
            0xE5 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::sbc); }
            0xF5 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, instruction_set::sbc); }
            0xF1 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, instruction_set::sbc); }
            0xED => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::sbc); }
            0xFD => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, instruction_set::sbc); }
            0xF9 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, instruction_set::sbc); }


            // STA
            0x81 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, NULL_READ, REG_WRITE, instruction_set::sta); }
            0x85 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  NULL_READ, REG_WRITE, instruction_set::sta); }
            0x95 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, NULL_READ, REG_WRITE, instruction_set::sta); }
            0x91 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, NULL_READ, REG_WRITE, instruction_set::sta); }
            0x8D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, NULL_READ, REG_WRITE, instruction_set::sta); }
            0x9D => { instruction_set::read_write_instruction_additional_delay(clock, pc_state, memory, &ADDR_ABX, NULL_READ, REG_WRITE, instruction_set::sta, pc_state::PcState::CYCLES_TO_CLOCK); }
            0x99 => { instruction_set::read_write_instruction_additional_delay(clock, pc_state, memory, &ADDR_ABY, NULL_READ, REG_WRITE, instruction_set::sta, pc_state::PcState::CYCLES_TO_CLOCK); }

            // SAX
            0x83 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, NULL_READ, REG_WRITE, instruction_set::sax); }
            0x87 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  NULL_READ, REG_WRITE, instruction_set::sax); }
            0x8F => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, NULL_READ, REG_WRITE, instruction_set::sax); }
            0x97 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPY, NULL_READ, REG_WRITE, instruction_set::sax); }

            // STX
            0x86 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  NULL_READ, REG_WRITE, instruction_set::stx); }
            0x96 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPY, NULL_READ, REG_WRITE, instruction_set::stx); }
            0x8E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, NULL_READ, REG_WRITE, instruction_set::stx); }

            // STY
            0x84 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  NULL_READ, REG_WRITE, instruction_set::sty); }
            0x94 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPY, NULL_READ, REG_WRITE, instruction_set::sty); }
            0x8C => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, NULL_READ, REG_WRITE, instruction_set::sty); }

            // JSR
            0x20 => { instruction_set::jump_sub_routine_instruction(clock, pc_state, memory); }

            // BPL case 0x10: if (self.pc_state.P.status.N == 0)
            0x10 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x80, 0x00); }
            // BMI case 0x30: if (self.pc_state.P.status.N == 1)
            0x30 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x80, 0x80); }
            // BVC case 0x50: if (self.pc_state.P.status.V == 0)
            0x50 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x40, 0x80); }
            // BVS case 0x70: if (self.pc_state.P.status.V == 1)
            0x70 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x40, 0x40); }
            // BCC case 0x90: if (self.pc_state.P.status.C == 0)
            0x90 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x01, 0x00); }
            // BCS case 0xB0: if (self.pc_state.P.status.C == 1)
            0xB0 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x01, 0x01); }
            // BNE case 0xD0: self.clocks += 2*CYCLES_TO_CLOCK if (self.pc_state.P.status.Z == 0)
            0xD0 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x02, 0x00); }
            // BEO case 0xF0: if (self.pc_state.P.status.Z == 1)
            0xF0 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x02, 0x02); }

            // RTS
            0x60 => { instruction_set::return_from_sub_routine_instruction(clock, pc_state, memory); }

            // PHA
            0x48 => { instruction_set::pha_instruction(clock, pc_state, memory); }

            // PLA
            0x68 => { instruction_set::pla_instruction(clock, pc_state, memory); }

            _ => {
                panic!("Opcode not implemented: 0x{:x}", op_code);
            }
        }
    }
}
