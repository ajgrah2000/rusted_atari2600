use super::super::clocks;
use super::super::memory::memory;
use super::super::memory::addressing;
use super::super::ports;
use super::pc_state;
use super::instruction_set;

pub struct Instruction {}

// There's likely a better way to specify the memory types, but this achieves the intent.
const ADDR_IMM: addressing::AddressingEnum = addressing::AddressingEnum::AddressingImmEnum;
const ADDR_ZP: addressing::AddressingEnum = addressing::AddressingEnum::AddressingZpEnum;
const ADDR_ZPX: addressing::AddressingEnum = addressing::AddressingEnum::AddressingZpxEnum;
const ADDR_ZPY: addressing::AddressingEnum = addressing::AddressingEnum::AddressingZpyEnum;
const ADDR_IZX: addressing::AddressingEnum = addressing::AddressingEnum::AddressingIzxEnum;
const ADDR_IZY: addressing::AddressingEnum = addressing::AddressingEnum::AddressingIzyEnum;

const ADDR_ABS: addressing::AddressingEnum = addressing::AddressingEnum::AddressingAbsEnum;
const ADDR_INDIRECT: addressing::AddressingEnum = addressing::AddressingEnum::AddressingIndirectEnum;
const ADDR_ABY: addressing::AddressingEnum = addressing::AddressingEnum::AddressingAbyEnum;
const ADDR_ABX: addressing::AddressingEnum = addressing::AddressingEnum::AddressingAbxEnum;
const ADDR_ACCUMULATOR: addressing::AddressingEnum = addressing::AddressingEnum::AddressingAccumulatorEnum;

// Page Delay version of addressing modes (only applicable to some indexed modes, that can carry).)
const ADDR_IZY_PAGE_DELAY: addressing::AddressingEnum = addressing::AddressingEnum::AddressingIZYPageDelayEnum;
const ADDR_ABY_PAGE_DELAY: addressing::AddressingEnum = addressing::AddressingEnum::AddressingAbyPageDelayEnum;
const ADDR_ABX_PAGE_DELAY: addressing::AddressingEnum = addressing::AddressingEnum::AddressingAbxPageDelayEnum;

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

enum O {
    Adc, And, Asl, Bit, Clc, Cld, Cli, Clv, Cmp, Cpx, Cpy, Dcp, Dec, Eor, Inc,
    Lda, Ldx, Ldy, Lsr, Nop, Or, Rol, Ror, Sax, Sbc, Sec, Sed, Sei, Sta, Stx,
    Sty, TNoStatus, TStatus,
    Jsr,
}

enum A {
    ImpliedAa, ImpliedXx, ImpliedYy, ImpliedNull, ImpliedXs, ImpliedSx, ImpliedXa, ImpliedAx, ImpliedAy, ImpliedYa,
    IzxR, IzyR,ImmR, ZpR, ZpyR, ZpxR, IzyDelayR, AbsR, AbxR, AbyR, AbxDelayR, AbyDelayR,
    ZpW, ZpxW, AbsW, AbxW, AbxDelayW, Acc, IzxRegW, ZpRegW, ZpxRegW, ZpyRegW, IzyRegW, AbsRegW,
    None,
}

impl Instruction {


    pub fn execute(
        op_code: u8,
        clock: &mut clocks::Clock,
        memory: &mut memory::Memory,
        pc_state: &mut pc_state::PcState,
        ports: &mut ports::Ports) {


        let op_fn = |op| match op {
            O::Adc => { instruction_set::adc }
            O::And => { instruction_set::and }
            O::Asl => {instruction_set::asl}
            O::Bit => { instruction_set::bit }
            O::Clc => {instruction_set::clc}
            O::Cld => {instruction_set::cld}
            O::Cli => {instruction_set::cli}
            O::Clv => {instruction_set::clv}
            O::Cmp => { instruction_set::cmp }
            O::Cpx => { instruction_set::cpx }
            O::Cpy => { instruction_set::cpy }
            O::Dcp => { instruction_set::dcp}
            O::Dec => {instruction_set::dec}
            O::Eor => { instruction_set::eor}
            O::Inc => {instruction_set::inc}
            O::Lda => { instruction_set::lda}
            O::Ldx => { instruction_set::ldx}
            O::Ldy => { instruction_set::ldy}
            O::Lsr => {instruction_set::lsr}
            O::Nop => {instruction_set::nop}
            O::Or => { instruction_set::or}
            O::Rol => { instruction_set::rol}
            O::Ror => { instruction_set::ror}
            O::Sax => { instruction_set::sax}
            O::Sbc => { instruction_set::sbc}
            O::Sec => {instruction_set::sec}
            O::Sed => {instruction_set::sed}
            O::Sei => {instruction_set::sei}
            O::Sta => { instruction_set::sta}
            O::Stx => { instruction_set::stx}
            O::Sty => { instruction_set::sty}
            O::TNoStatus => { instruction_set::t_no_status }
            O::TStatus => { instruction_set::t_status }
            _ => { panic!("Unexpected operator"); }
        };

        let mut op = |op_arg, addr| {
            match (addr, op_arg) {
                (A::ImpliedAa, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_A, op_fn(o)); }
                (A::ImpliedXx, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_X, op_fn(o)); }
                (A::ImpliedYy, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_Y, WRITE_REG_Y, op_fn(o)); }
                (A::ImpliedNull, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_NULL, WRITE_NULL, op_fn(o)); }

                (A::ImpliedXs, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_S, op_fn(o)); }
                (A::ImpliedSx, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_S, WRITE_REG_X, op_fn(o)); }
                (A::ImpliedXa, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_X, WRITE_REG_A, op_fn(o)); }
                (A::ImpliedAx, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_X, op_fn(o)); }
                (A::ImpliedAy, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_A, WRITE_REG_Y, op_fn(o)); }
                (A::ImpliedYa, o) => {instruction_set::single_byte_instruction(clock, pc_state, memory, READ_REG_Y, WRITE_REG_A, op_fn(o)); }
                (A::IzxR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::IzyR, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::ImmR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::ZpR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::ZpxR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::ZpyR, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPY, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::IzyDelayR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::AbsR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::AbxR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::AbyR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::AbxDelayR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::AbyDelayR, o) => {instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABY_PAGE_DELAY, MEMORY_READ, MEMORY_NULL, op_fn(o)); }
                (A::ZpW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP, MEMORY_READ, MEMORY_WRITE, op_fn(o)); }
                (A::ZpxW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, MEMORY_READ, MEMORY_WRITE, op_fn(o)); }
                (A::AbsW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, MEMORY_READ, MEMORY_WRITE, op_fn(o)); }
                (A::AbxDelayW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX_PAGE_DELAY, MEMORY_READ, MEMORY_WRITE, op_fn(o)); }
                (A::AbxW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABX, MEMORY_READ, MEMORY_WRITE, op_fn(o)); }
                (A::Acc, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ACCUMULATOR, ACCUMULATOR_READ, ACCUMULATOR_WRITE, op_fn(o)); }
                (A::IzxRegW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, NULL_READ, REG_WRITE, op_fn(o)); }
                (A::ZpRegW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  NULL_READ, REG_WRITE, op_fn(o)); }
                (A::ZpxRegW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, NULL_READ, REG_WRITE, op_fn(o)); }
                (A::IzyRegW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, NULL_READ, REG_WRITE, op_fn(o)); }
                (A::AbsRegW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ABS, NULL_READ, REG_WRITE, op_fn(o)); }
                (A::ZpyRegW, o) => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPY, NULL_READ, REG_WRITE, op_fn(o)); }

                (A::None, O::Jsr) => { instruction_set::jump_sub_routine_instruction(clock, pc_state, memory); }
                _ => { panic!("Unexpected address operator combination")}
            }
        };

        match op_code {

            0xEA => { op(O::Nop, A::ImpliedAa); }

            0x0A => { op(O::Asl, A::ImpliedAa); }
            0x4A => { op(O::Lsr, A::ImpliedAa); }
            0xE8 => { op(O::Inc, A::ImpliedXx); }
            0xC8 => { op(O::Inc, A::ImpliedYy); }
            0xCA => { op(O::Dec, A::ImpliedXx); }
            0x88 => { op(O::Dec, A::ImpliedYy); }

            0x18 => { op(O::Clc, A::ImpliedNull); }
            0xD8 => { op(O::Cld, A::ImpliedNull); }
            0x58 => { op(O::Cli, A::ImpliedNull); }
            0xB8 => { op(O::Clv, A::ImpliedNull); }

            0x38 => { op(O::Sec, A::ImpliedNull); }
            0x78 => { op(O::Sei, A::ImpliedNull); }
            0xF8 => { op(O::Sed, A::ImpliedNull); }

            // Break instruction, software 'interrupt'
            0x00 => { instruction_set::break_instruction(clock, pc_state, memory); }

            // Register Transfers
            0x9A => { op(O::TNoStatus, A::ImpliedXs); }
            0xBA => { op(O::TNoStatus, A::ImpliedSx); }
            0x8A => { op(O::TStatus, A::ImpliedXa); }
            0xAA => { op(O::TStatus, A::ImpliedAx); }
            0xA8 => { op(O::TStatus, A::ImpliedAy); }
            0x98 => { op(O::TStatus, A::ImpliedYa); }

            // ADC
            0x61 => { op(O::Adc, A::IzxR); }
            0x69 => { op(O::Adc, A::ImmR); }
            0x65 => { op(O::Adc, A::ZpR); }
            0x75 => { op(O::Adc, A::ZpxR); }
            0x71 => { op(O::Adc, A::IzyDelayR); }
            0x6D => { op(O::Adc, A::AbsR); }
            0x7D => { op(O::Adc, A::AbxDelayR); }
            0x79 => { op(O::Adc, A::AbyDelayR); }

            // ASL
            0x06 => { op(O::Asl, A::ZpW); }
            0x16 => { op(O::Asl, A::ZpxW); }
            0x0E => { op(O::Asl, A::AbsW); }
            0x1E => { op(O::Asl, A::AbxDelayW); }

            // AND
            0x21 => { op(O::And, A::IzxR); }
            0x29 => { op(O::And, A::ImmR); }
            0x25 => { op(O::And, A::ZpR); }
            0x35 => { op(O::And, A::ZpxR); }
            0x31 => { op(O::And, A::IzyDelayR); }
            0x2D => { op(O::And, A::AbsR); }
            0x3D => { op(O::And, A::AbxDelayR); }
            0x39 => { op(O::And, A::AbyDelayR); }

            // CPX
            0xE0 => { op(O::Cpx, A::ImmR); }
            0xE4 => { op(O::Cpx, A::ZpR); }
            0xEC => { op(O::Cpx, A::AbsR); }

            // CPY
            0xC0 => { op(O::Cpy, A::ImmR); }
            0xC4 => { op(O::Cpy, A::ZpR); }
            0xCC => { op(O::Cpy, A::AbsR); }

            // BIT
            0x24 => { op(O::Bit, A::ZpR); }
            0x2C => { op(O::Bit, A::AbsR); }

            // CMP
            0xC1 => { op(O::Cmp, A::IzxR); }
            0xC9 => { op(O::Cmp, A::ImmR); }
            0xC5 => { op(O::Cmp, A::ZpR); }
            0xD5 => { op(O::Cmp, A::ZpxR); }
            0xD1 => { op(O::Cmp, A::IzyDelayR); }
            0xCD => { op(O::Cmp, A::AbsR); }
            0xDD => { op(O::Cmp, A::AbxDelayR); }
            0xD9 => { op(O::Cmp, A::AbyDelayR); }

            // DEC
            0xC6 => { op(O::Dec, A::ZpW); }
            0xD6 => { op(O::Dec, A::ZpxW); }
            0xCE => { op(O::Dec, A::AbsW); }
            0xDE => { op(O::Dec, A::AbxDelayW); }

            // EOR
            0x41 => { op(O::Eor, A::IzxR); }
            0x49 => { op(O::Eor, A::ImmR); }
            0x45 => { op(O::Eor, A::ZpR); }
            0x55 => { op(O::Eor, A::ZpxR); }
            0x51 => { op(O::Eor, A::IzyR); }
            0x4D => { op(O::Eor, A::AbsR); }
            0x5D => { op(O::Eor, A::AbxR); }
            0x59 => { op(O::Eor, A::AbyR); }

            // INC
            0xE6 => { op(O::Inc, A::ZpW); }
            0xF6 => { op(O::Inc, A::ZpxW); }
            0xEE => { op(O::Inc, A::AbsW); }
            0xFE => { op(O::Inc, A::AbxDelayW); }

            // LDA
            0xA1 => { op(O::Lda, A::IzxR); }
            0xA9 => { op(O::Lda, A::ImmR); }
            0xA5 => { op(O::Lda, A::ZpR); }
            0xB5 => { op(O::Lda, A::ZpxR); }
            0xB1 => { op(O::Lda, A::IzyDelayR); }
            0xAD => { op(O::Lda, A::AbsR); }
            0xBD => { op(O::Lda, A::AbxDelayR); }
            0xB9 => { op(O::Lda, A::AbyDelayR); }

            // LDX
            0xA2 => { op(O::Ldx, A::ImmR); }
            0xA6 => { op(O::Ldx, A::ZpR); }
            0xB6 => { op(O::Ldx, A::ZpyR); }
            0xAE => { op(O::Ldx, A::AbsR); }
            0xBE => { op(O::Ldx, A::AbyDelayR); }

            // LDY
            0xA0 => { op(O::Ldy, A::ImmR); }
            0xA4 => { op(O::Ldy, A::ZpR); }
            0xB4 => { op(O::Ldy, A::ZpxR); }
            0xAC => { op(O::Ldy, A::AbsR); }
            0xBC => { op(O::Ldy, A::AbxDelayR); }

            // LSR
            0x46 => { op(O::Lsr, A::ZpW); }
            0x56 => { op(O::Lsr, A::ZpxW); }
            0x4E => { op(O::Lsr, A::AbsW); }
            0x5E => { op(O::Lsr, A::AbxDelayW); }

            // OR
            0x01 => { op(O::Or, A::IzxR); }
            0x09 => { op(O::Or, A::ImmR); }
            0x05 => { op(O::Or, A::ZpR); }
            0x15 => { op(O::Or, A::ZpxR); }
            0x11 => { op(O::Or, A::IzyDelayR); }
            0x0D => { op(O::Or, A::AbsR); }
            0x1D => { op(O::Or, A::AbxDelayR); }
            0x19 => { op(O::Or, A::AbyDelayR); }

            // ROL 
            // TODO: Page delays (need to make sure separation of read/write)
            0x26 => { op(O::Rol, A::ZpW); }
            0x36 => { op(O::Rol, A::ZpxW); }
            0x2E => { op(O::Rol, A::AbsW); }
            0x3E => { op(O::Rol, A::AbxW); }
            0x2A => { op(O::Rol, A::Acc); }

            // ROR
            // TODO: Page delays (need to make sure separation of read/write)
            0x66 => { op(O::Ror, A::ZpW); }
            0x76 => { op(O::Ror, A::ZpxW); }
            0x6E => { op(O::Ror, A::AbsW); }
            0x7E => { op(O::Ror, A::AbxW); }
            0x6A => { op(O::Ror, A::Acc); }

            // SBC
            0xE1 => { op(O::Sbc, A::IzxR); }
            0xE9 => { op(O::Sbc, A::ImmR); }
            0xE5 => { op(O::Sbc, A::ZpR); }
            0xF5 => { op(O::Sbc, A::ZpxR); }
            0xF1 => { op(O::Sbc, A::IzyDelayR); }
            0xED => { op(O::Sbc, A::AbsR); }
            0xFD => { op(O::Sbc, A::AbxDelayR); }
            0xF9 => { op(O::Sbc, A::AbyDelayR); }

            // STA
            0x81 => { op(O::Sta, A::IzxRegW); }
            0x85 => { op(O::Sta, A::ZpRegW); }
            0x95 => { op(O::Sta, A::ZpxRegW); }
            0x91 => { op(O::Sta, A::IzyRegW); }
            0x8D => { op(O::Sta, A::AbsRegW); }
            0x9D => { instruction_set::read_write_instruction_additional_delay(clock, pc_state, memory, &ADDR_ABX, NULL_READ, REG_WRITE, instruction_set::sta, pc_state::PcState::CYCLES_TO_CLOCK); }
            0x99 => { instruction_set::read_write_instruction_additional_delay(clock, pc_state, memory, &ADDR_ABY, NULL_READ, REG_WRITE, instruction_set::sta, pc_state::PcState::CYCLES_TO_CLOCK); }

            // SAX
            0x83 => { op(O::Sax, A::IzxRegW); }
            0x87 => { op(O::Sax, A::ZpRegW); }
            0x8F => { op(O::Sax, A::AbsRegW); }
            0x97 => { op(O::Sax, A::ZpyRegW); }

            // STX
            0x86 => { op(O::Stx, A::ZpRegW); }
            0x96 => { op(O::Stx, A::ZpyRegW); }
            0x8E => { op(O::Stx, A::AbsRegW); }

            // STY
            0x84 => { op(O::Sty, A::ZpRegW); }
            0x94 => { op(O::Sty, A::ZpxRegW); }
            0x8C => { op(O::Sty, A::AbsRegW); }

            // DCP
            // Undocumented instruction
            0xC3 => { op(O::Dcp, A::IzxR); }
            0xC7 => { op(O::Dcp, A::ZpR); }
            0xD7 => { op(O::Dcp, A::ZpxR); }
            0xD3 => { op(O::Dcp, A::IzyDelayR); }
            0xCF => { op(O::Dcp, A::AbsR); }
            0xDF => { op(O::Dcp, A::AbxDelayR); }
            0xDB => { op(O::Dcp, A::AbyDelayR); }

            // JSR
            0x20 => { op(O::Jsr, A::None); }

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
                panic!("Ocode not implemented: 0x{:x}", op_code);
            }
        }
    }
}

#[cfg(test)]
mod tests {
}
