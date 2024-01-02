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
const ADDR_IZY:addressing::AddressingIZY = addressing::AddressingIZY::new(false);

const ADDR_ABS:addressing::AddressingAbs = addressing::AddressingAbs::new();
const ADDR_INDIRECT:addressing::AddressingIndirect = addressing::AddressingIndirect::new();
const ADDR_ABY:addressing::AddressingAby = addressing::AddressingAby::new();
const ADDR_ABX:addressing::AddressingAbx = addressing::AddressingAbx::new();
const ADDR_ACCUMULATOR:addressing::AddressingAccumulator = addressing::AddressingAccumulator::new();

// Page Delay version of addressing modes (only applicable to some indexed modes, that can carry).)
const ADDR_IZY_PAGE_DELAY:addressing::AddressingIZY = addressing::AddressingIZY::new(true);
const ADDR_ABY_PAGE_DELAY:addressing::AddressingAbyPageDelay = addressing::AddressingAbyPageDelay::new();
const ADDR_ABX_PAGE_DELAY:addressing::AddressingAbxPageDelay = addressing::AddressingAbxPageDelay::new();

const NULL_READ:addressing::NullRead = addressing::NullRead::new();
const MEMORY_READ:addressing::MemoryRead = addressing::MemoryRead::new();
const ACCUMULATOR_READ:addressing::AccumulatorRead = addressing::AccumulatorRead::new();
const MEMORY_WRITE:addressing::MemoryWrite = addressing::MemoryWrite::new();
const ACCUMULATOR_WRITE:addressing::AccumulatorWrite = addressing::AccumulatorWrite::new();
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

        // Op code, 0xFF -> 0bAAABBBCC
        let get_a = |op:u8| op >> 5 & 0x7 as u8;
        let get_b = |op:u8| op >> 2 & 0x7 as u8;
        let get_c = |op:u8| op & 0x3 as u8;

        let op_instruction = |op: u8| 
            match op {
                n if get_c(n) == 0x1 => {
                    // Get the op-code 'instruction' first.
                    match get_a(op) {
                        0x0 =>  {instruction_set::or},
                        0x1 =>  {instruction_set::and},
                        0x2 =>  {instruction_set::eor},
                        0x3 =>  {instruction_set::adc},
                        0x4 =>  {instruction_set::sta},
                        0x5 =>  {instruction_set::lda},
                        0x6 =>  {instruction_set::cmp},
                        0x7 =>  {instruction_set::sbc},
                        _ => {panic!("Not possible");},
                    }
                },
                n if (get_c(n) == 0x2) && ((get_b(n) & 0x1 == 0x1)) => {
                    match n >> 5 & 7 {
                        0 => {instruction_set::asl},
                        1 => {instruction_set::rol},
                        2 => {instruction_set::lsr},
                        3 => {instruction_set::ror},
                        5 => {instruction_set::ldx},
                        6 => {instruction_set::dec},
                        7 => {instruction_set::inc},
                        _ => {panic!("Opcode not implemented: 0x{:x}", op);}
                    }
                }
                n if (get_c(n) == 0x2) && (get_b(n) == 0x2) /* register versions */ => {
                    match n >> 5 & 7 {
                        0 => {instruction_set::asl},
                        1 => {instruction_set::rol},
                        2 => {instruction_set::lsr},
                        3 => {instruction_set::ror},
                        6 => {instruction_set::dec},
                        /* TAX */
                        /* NOP */
                        _ => {panic!("Opcode not implemented: 0x{:x}", op);}
                    }
                }
                n if (get_c(n) == 0x2) && ((get_b(n) == 0x0)) => {
                    match n >> 5 & 7 {
                        5 => {instruction_set::ldx},
                        _ => {panic!("Opcode not implemented: 0x{:x}", op);},
                    }
                }
                _ => {panic!("Opcode not implemented: 0x{:x}", op);}
            };
                               
        match op_code {

            0xEA => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_A, instruction_set::nop); }

            0x0A => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_A, op_instruction(op_code)); }
            0x4A => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_A, op_instruction(op_code)); }
            0xE8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_X, instruction_set::inc); } /* INX */
            0xC8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_Y, WRITE_REG_Y, instruction_set::inc); } /* INY */
            0xCA => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_X, op_instruction(op_code)); } /* DEX */
            0x88 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_Y, WRITE_REG_Y, instruction_set::dec); } /* DEY */

            0x18 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::clc); }
            0xD8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::cld); }
            0x58 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::cli); }
            0xB8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::clv); }

            0x38 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::sec); }
            0x78 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::sei); }
            0xF8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, instruction_set::sed); }

            // Break instruction, software 'interrupt'
            0x00 => { instruction_set::break_instruction(clock, pc_state, memory); }

            // Register Transfers
            0x9A => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_S, instruction_set::t_no_status); }
            0xBA => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_S, WRITE_REG_X, instruction_set::t_no_status); }
            0x8A => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_A, instruction_set::t_status); }
            0xAA => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_X, instruction_set::t_status); }
            0xA8 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_Y, instruction_set::t_status); }
            0x98 => { instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_Y, WRITE_REG_A, instruction_set::t_status); }

            n if n & 0x3 == 0x1 => {
                match op_code {
                    // EOR
                    0x41 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    0x49 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    0x45 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    0x55 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    0x51 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    0x4D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    0x5D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    0x59 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }

                    // STA
                    0x81 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, NULL_READ, REG_WRITE, op_instruction(op_code)); }
                    0x85 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  NULL_READ, REG_WRITE, op_instruction(op_code)); }
                    0x95 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, NULL_READ, REG_WRITE, op_instruction(op_code)); }
                    0x91 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, NULL_READ, REG_WRITE, op_instruction(op_code)); }
                    0x8D => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, NULL_READ, REG_WRITE, op_instruction(op_code)); }
                    0x9D => { instruction_set::read_write_instruction_additional_delay(clock, pc_state, memory, &ADDR_ABX, NULL_READ, REG_WRITE, op_instruction(op_code), pc_state::PcState::CYCLES_TO_CLOCK); }
                    0x99 => { instruction_set::read_write_instruction_additional_delay(clock, pc_state, memory, &ADDR_ABY, NULL_READ, REG_WRITE, op_instruction(op_code), pc_state::PcState::CYCLES_TO_CLOCK); }

                    // OR, AND, ADC, LDA, CMP, SBC
                    n if 0x0 == get_b(n) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX,            MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    n if 0x1 == get_b(n) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,             MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    n if 0x2 == get_b(n) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM,            MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    n if 0x3 == get_b(n) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,            MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    n if 0x4 == get_b(n) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    n if 0x5 == get_b(n) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,            MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    n if 0x6 == get_b(n) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
                    n if 0x7 == get_b(n) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }

                    _ => { panic!("Unmatched (0xXXXXXX01) opcode 0x{:x}", op_code); }
                }
            },

            // ASL
            0x06 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,             MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0x16 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,            MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0x0E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,            MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0x1E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }

            // CPX
            0xE0 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::cpx); }
            0xE4 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::cpx); }
            0xEC => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::cpx); }

            // CPY
            0xC0 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::cpy); }
            0xC4 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::cpy); }
            0xCC => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::cpy); }

            // BIT
            0x24 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  MEMORY_READ, MEMORY_NULL, instruction_set::bit); }
            0x2C => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, instruction_set::bit); }

            // DEC
            0xC6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,             MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0xD6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,            MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0xCE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,            MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0xDE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }

            // INC
            0xE6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,             MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0xF6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,            MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0xEE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,            MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0xFE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }

            // LDX
            0xA2 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM,            MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
            0xA6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,             MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
            0xB6 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPY,            MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
            0xAE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,            MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }
            0xBE => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, op_instruction(op_code)); }

            // LDY
            0xA0 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM,            MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }
            0xA4 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,             MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }
            0xB4 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,            MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }
            0xAC => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,            MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }
            0xBC => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, instruction_set::ldy); }

            // LSR
            0x46 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,             MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0x56 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,            MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0x4E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,            MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }
            0x5E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_WRITE, op_instruction(op_code)); }

            // ROL 
            // TODO: Page delays (need to make sure separation of read/write)
            0x26 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,          MEMORY_READ,      MEMORY_WRITE,      op_instruction(op_code)); }
            0x36 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,         MEMORY_READ,      MEMORY_WRITE,      op_instruction(op_code)); }
            0x2E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,         MEMORY_READ,      MEMORY_WRITE,      op_instruction(op_code)); }
            0x3E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX,         MEMORY_READ,      MEMORY_WRITE,      op_instruction(op_code)); }
            0x2A => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ACCUMULATOR, ACCUMULATOR_READ, ACCUMULATOR_WRITE, op_instruction(op_code)); }

            // ROR
            // TODO: Page delays (need to make sure separation of read/write)
            0x66 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,          MEMORY_READ,      MEMORY_WRITE,      op_instruction(op_code)); }
            0x76 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,         MEMORY_READ,      MEMORY_WRITE,      op_instruction(op_code)); }
            0x6E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,         MEMORY_READ,      MEMORY_WRITE,      op_instruction(op_code)); }
            0x7E => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX,         MEMORY_READ,      MEMORY_WRITE,      op_instruction(op_code)); }
            0x6A => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ACCUMULATOR, ACCUMULATOR_READ, ACCUMULATOR_WRITE, op_instruction(op_code)); }

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
            0x94 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, NULL_READ, REG_WRITE, instruction_set::sty); }
            0x8C => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, NULL_READ, REG_WRITE, instruction_set::sty); }

            // DCP
            // Undocumented instruction
            0xC3 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX,            MEMORY_READ, MEMORY_NULL, instruction_set::dcp); }
            0xC7 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,             MEMORY_READ, MEMORY_NULL, instruction_set::dcp); }
            0xD7 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX,            MEMORY_READ, MEMORY_NULL, instruction_set::dcp); }
            0xD3 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, instruction_set::dcp); }
            0xCF => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS,            MEMORY_READ, MEMORY_NULL, instruction_set::dcp); }
            0xDF => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, instruction_set::dcp); }
            0xDB => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, instruction_set::dcp); }

            // JSR
            0x20 => { instruction_set::jump_sub_routine_instruction(clock, pc_state, memory); }

            // BPL case 0x10: if (self.pc_state.P.status.N == 0)
            0x10 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x80, 0x00); }
            // BMI case 0x30: if (self.pc_state.P.status.N == 1)
            0x30 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x80, 0x80); }
            // BVC case 0x50: if (self.pc_state.P.status.V == 0)
            0x50 => { instruction_set::branch_instruction(clock, pc_state, memory, 0x40, 0x00); }
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

            0x40 => { instruction_set::return_from_interrupt(clock, pc_state, memory); }
            // RTS
            0x60 => { instruction_set::return_from_sub_routine_instruction(clock, pc_state, memory); }

            // JMP, absolute (effectively immediate)
            0x4C => { instruction_set::jump_instruction(clock, pc_state, memory, &ADDR_ABS); }
            // JMP, absolute (effectively absolute)
            0x6C => { instruction_set::jump_instruction(clock, pc_state, memory, &ADDR_INDIRECT); }

            // PHP
            0x08 => { instruction_set::php_instruction(clock, pc_state, memory); }

            // PLP
            0x28 => { instruction_set::plp_instruction(clock, pc_state, memory); }

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

#[cfg(test)]
mod tests {
}
