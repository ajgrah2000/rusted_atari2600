use super::super::clocks;
use super::super::memory::memory;
use super::super::memory::addressing;
use super::super::ports;
use super::pc_state;
use super::instruction_set;

pub struct Instruction {}

use addressing::AddressingEnum::*;

// Page Delay version of addressing modes (only applicable to some indexed modes, that can carry).)
const NULL_READ:addressing::NullRead = addressing::NullRead::new();
const MEMORY_READ:addressing::MemoryRead = addressing::MemoryRead::new();
const ACCUMULATOR_READ:addressing::AccumulatorRead = addressing::AccumulatorRead::new();
const MEMORY_WRITE:addressing::MemoryWrite = addressing::MemoryWrite::new();
const ACCUMULATOR_WRITE:addressing::AccumulatorWrite = addressing::AccumulatorWrite::new();
const REG_WRITE:addressing::RegisterWrite = addressing::RegisterWrite::new();
const MEMORY_NULL:addressing::MemoryNull = addressing::MemoryNull::new();

use OpName::*;
use AddressMode::*;

enum ReadType { ReadNull, ReadX, ReadY, ReadA, ReadS }
enum WriteType { WriteNull, WriteX, WriteY, WriteA, WriteS }

enum OpName {
    Adc, And, Asl, Bit, Clc, Cld, Cli, Clv, Cmp, Cpx, Cpy, Dcp, Dec, Eor, Inc,
    Lda, Ldx, Ldy, Lsr, Nop, Or, Rol, Ror, Sax, Sbc, Sec, Sed, Sei, Sta, Stx,
    Sty, TNoStatus, TStatus,
    Jsr, Brk, Rti, Rts, JmpAbs, JmpInd, Php, Plp, Pha, Pla, Bpl, Bmi, Bvc, Bvs, Bcc, Bcs, Bne, Beo,
    NoOP
}


enum AddressMode {
    Imp(ReadType, WriteType),
    IzxR, IzyR,ImmR, ZpR, ZpyR, ZpxR, IzyDelayR, AbsR, AbxR, AbyR, AbxDelayR, AbyDelayR,
    ZpW, ZpxW, AbsW, AbxW, AbxDelayW, Acc, IzxRegW, ZpRegW, ZpxRegW, ZpyRegW, IzyRegW, AbsRegW,
    None, AbxRegWDelay, AbyRegWDelay,
    NoA
}

impl Instruction {

    pub fn execute(
        op_code: u8,
        clock: &mut clocks::Clock,
        memory: &mut memory::Memory,
        pc_state: &mut pc_state::PcState,
        ports: &mut ports::Ports) {

        let op_fn = |op| match op {
            Adc => instruction_set::adc,
            And => instruction_set::and,
            Asl => instruction_set::asl,
            Bit => instruction_set::bit,
            Clc => instruction_set::clc,
            Cld => instruction_set::cld,
            Cli => instruction_set::cli,
            Clv => instruction_set::clv,
            Cmp => instruction_set::cmp,
            Cpx => instruction_set::cpx,
            Cpy => instruction_set::cpy,
            Dcp => instruction_set::dcp,
            Dec => instruction_set::dec,
            Eor => instruction_set::eor,
            Inc => instruction_set::inc,
            Lda => instruction_set::lda,
            Ldx => instruction_set::ldx,
            Ldy => instruction_set::ldy,
            Lsr => instruction_set::lsr,
            Nop => instruction_set::nop,
            Or => instruction_set::or,
            Rol => instruction_set::rol,
            Ror => instruction_set::ror,
            Sax => instruction_set::sax,
            Sbc => instruction_set::sbc,
            Sec => instruction_set::sec,
            Sed => instruction_set::sed,
            Sei => instruction_set::sei,
            Sta => instruction_set::sta,
            Stx => instruction_set::stx,
            Sty => instruction_set::sty,
            TNoStatus => instruction_set::t_no_status,
            TStatus => instruction_set::t_status,
            _ => {panic!("Unexpected operator");}
        };

        let read_fn = |read_type| match read_type {
            ReadType::ReadNull => pc_state::read_null,
            ReadType::ReadX => pc_state::read_x,
            ReadType::ReadY => pc_state::read_y,
            ReadType::ReadA => pc_state::read_a,
            ReadType::ReadS => pc_state::read_s,
        };

        let write_fn = |write_type| match write_type {
            WriteType::WriteNull => pc_state::write_null,
            WriteType::WriteX => pc_state::write_x,
            WriteType::WriteY => pc_state::write_y,
            WriteType::WriteA => pc_state::write_a,
            WriteType::WriteS => pc_state::write_s,
        };

        let mut op = |op_arg, addr| {
            match (addr, op_arg) {
                (Imp(r, w), o) => instruction_set::single_byte_instruction(clock, pc_state, memory, read_fn(r), write_fn(w), op_fn(o)),

                (IzxR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingIzxEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (IzyR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingIzyEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (ImmR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingImmEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (ZpR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingZpEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (ZpxR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingZpxEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (ZpyR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingZpyEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (IzyDelayR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingIZYPageDelayEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (AbsR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbsEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (AbxR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbxEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (AbyR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbyEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (AbxDelayR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbxPageDelayEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (AbyDelayR, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbyPageDelayEnum, MEMORY_READ, MEMORY_NULL, op_fn(o)),
                (ZpW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingZpEnum, MEMORY_READ, MEMORY_WRITE, op_fn(o)),
                (ZpxW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingZpxEnum, MEMORY_READ, MEMORY_WRITE, op_fn(o)),
                (AbsW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbsEnum, MEMORY_READ, MEMORY_WRITE, op_fn(o)),
                (AbxDelayW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbxPageDelayEnum, MEMORY_READ, MEMORY_WRITE, op_fn(o)),
                (AbxW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbxEnum, MEMORY_READ, MEMORY_WRITE, op_fn(o)),
                (Acc, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAccumulatorEnum, ACCUMULATOR_READ, ACCUMULATOR_WRITE, op_fn(o)),
                (IzxRegW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingIzxEnum, NULL_READ, REG_WRITE, op_fn(o)),
                (ZpRegW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingZpEnum,  NULL_READ, REG_WRITE, op_fn(o)),
                (ZpxRegW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingZpxEnum, NULL_READ, REG_WRITE, op_fn(o)),
                (IzyRegW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingIzyEnum, NULL_READ, REG_WRITE, op_fn(o)),
                (AbsRegW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingAbsEnum, NULL_READ, REG_WRITE, op_fn(o)),
                (ZpyRegW, o) => instruction_set::read_write_instruction(clock, pc_state, memory, &AddressingZpyEnum, NULL_READ, REG_WRITE, op_fn(o)),

                (AbxRegWDelay, o) => instruction_set::read_write_instruction_additional_delay(clock, pc_state, memory, &AddressingAbxEnum, NULL_READ, REG_WRITE, op_fn(o), pc_state::PcState::CYCLES_TO_CLOCK),
                (AbyRegWDelay, o) => instruction_set::read_write_instruction_additional_delay(clock, pc_state, memory, &AddressingAbyEnum, NULL_READ, REG_WRITE, op_fn(o), pc_state::PcState::CYCLES_TO_CLOCK),

                (None, Jsr) => instruction_set::jump_sub_routine_instruction(clock, pc_state, memory),
                (None, Brk) => instruction_set::break_instruction(clock, pc_state, memory),
                (None, Rti) => instruction_set::return_from_interrupt(clock, pc_state, memory),
                (None, Rts) => instruction_set::return_from_sub_routine_instruction(clock, pc_state, memory),
                (None, JmpAbs) => instruction_set::jump_instruction(clock, pc_state, memory, &AddressingAbsEnum),
                (None, JmpInd) => instruction_set::jump_instruction(clock, pc_state, memory, &AddressingIndirectEnum),
                (None, Php) => instruction_set::php_instruction(clock, pc_state, memory),
                (None, Plp) => instruction_set::plp_instruction(clock, pc_state, memory),
                (None, Pha) => instruction_set::pha_instruction(clock, pc_state, memory),
                (None, Pla) => instruction_set::pla_instruction(clock, pc_state, memory),
                (None, Bpl) => instruction_set::branch_instruction(clock, pc_state, memory, 0x80, 0x00), // N == 0
                (None, Bmi) => instruction_set::branch_instruction(clock, pc_state, memory, 0x80, 0x80), // N == 1
                (None, Bvc) => instruction_set::branch_instruction(clock, pc_state, memory, 0x40, 0x00), // V == 0
                (None, Bvs) => instruction_set::branch_instruction(clock, pc_state, memory, 0x40, 0x40), // V == 1
                (None, Bcc) => instruction_set::branch_instruction(clock, pc_state, memory, 0x01, 0x00), // C == 0
                (None, Bcs) => instruction_set::branch_instruction(clock, pc_state, memory, 0x01, 0x01), // C == 1
                (None, Bne) => instruction_set::branch_instruction(clock, pc_state, memory, 0x02, 0x00), // Z == 0
                (None, Beo) => instruction_set::branch_instruction(clock, pc_state, memory, 0x02, 0x02), // Z == 1

                _ => { panic!("Unexpected address operator combination")}
            }
        };

        let mut low =  |(op0, a0), (op1, a1), (op2, a2), (op3, a3), (op4, a4), (op5, a5), (op6, a6), (op7, a7)| { 
            match op_code & 0x7 {
                0 => op(op0, a0), 
                1 => op(op1, a1), 
                2 => op(op2, a2),
                3 => op(op3, a3),
                4 => op(op4, a4),
                5 => op(op5, a5),
                6 => op(op6, a6),
                7 => op(op7, a7), 
                _ => panic!("Not Possible")
            }
        };

        match op_code & 0xF8 {

            0x00 => low((Brk, Imp(ReadType::ReadNull, WriteType::WriteNull)),  (Or, IzxR),         (NoOP,  NoA),      (NoOP, NoA),     (NoOP, NoA),     (Or, ZpR),          (Asl, ZpW),      (NoOP, NoA)),
            0x08 => low((Php, None),     (Or, ImmR),         (Asl, Imp(ReadType::ReadA, WriteType::WriteA)),      (NoOP, NoA),     (NoOP, NoA),     (Or, AbsR),         (Asl, AbsW),     (NoOP, NoA)),
            0x10 => low((Bpl, None),     (Or, IzyDelayR),    (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Or, ZpxR),         (Asl, ZpxW),     (NoOP, NoA)),
            0x18 => low((Clc, Imp(ReadType::ReadNull, WriteType::WriteNull)),  (Or, AbyDelayR),    (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Or, AbxDelayR),    (Asl, AbxDelayW),(NoOP, NoA)),
            0x20 => low((Jsr, None),     (And, IzxR),        (NoOP, NoA),       (NoOP, NoA),     (Bit, ZpR),      (And, ZpR),         (Rol, ZpW),      (NoOP, NoA)),
            0x28 => low((Plp, None),     (And, ImmR),        (Rol, Acc),        (NoOP, NoA),     (Bit, AbsR),     (And, AbsR),        (Rol, AbsW),     (NoOP, NoA)),
            0x30 => low((Bmi, None),     (And, IzyDelayR),   (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (And, ZpxR),        (Rol, ZpxW),     (NoOP, NoA)),
            0x38 => low((Sec, Imp(ReadType::ReadNull, WriteType::WriteNull)),  (And, AbyDelayR),   (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (And, AbxDelayR),   (Rol, AbxW),     (NoOP, NoA)),
            0x40 => low((Rti, None),     (Eor, IzxR),        (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Eor, ZpR),         (Lsr, ZpW),      (NoOP, NoA)),
            0x48 => low((Pha, None),     (Eor, ImmR),        (Lsr, Imp(ReadType::ReadA, WriteType::WriteA)),      (NoOP, NoA),     (JmpAbs, None),  (Eor, AbsR),        (Lsr, AbsW),     (NoOP, NoA)),
            0x50 => low((Bvc, None),     (Eor, IzyR),        (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Eor, ZpxR),        (Lsr, ZpxW),     (NoOP, NoA)),
            0x58 => low((Cli, Imp(ReadType::ReadNull, WriteType::WriteNull)),  (Eor, AbyR),        (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Eor, AbxR),        (Lsr, AbxDelayW),(NoOP, NoA)),
            0x60 => low((Rts, None),     (Adc, IzxR),        (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Adc, ZpR),         (Ror, ZpW),      (NoOP, NoA)),
            0x68 => low((Pla, None),     (Adc, ImmR),        (Ror, Acc),        (NoOP, NoA),     (JmpInd, None),  (Adc, AbsR),        (Ror, AbsW),     (NoOP, NoA)),
            0x70 => low((Bvs, None),     (Adc, IzyDelayR),   (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Adc, ZpxR),        (Ror, ZpxW),     (NoOP, NoA)),
            0x78 => low((Sei, Imp(ReadType::ReadNull, WriteType::WriteNull)),  (Adc, AbyDelayR),   (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Adc, AbxDelayR),   (Ror, AbxW),     (NoOP, NoA)),
            0x80 => low((NoOP, NoA),     (Sta, IzxRegW),     (NoOP, NoA),       (Sax, IzxRegW),  (Sty, ZpRegW),   (Sta, ZpRegW),      (Stx, ZpRegW),   (Sax, ZpRegW)),
            0x88 => low((Dec, Imp(ReadType::ReadY, WriteType::WriteY)),    (NoOP, NoA),        (TStatus, Imp(ReadType::ReadX, WriteType::WriteA)),  (NoOP, NoA),     (Sty, AbsRegW),  (Sta, AbsRegW),     (Stx, AbsRegW),  (Sax, AbsRegW)),
            0x90 => low((Bcc, None),     (Sta, IzyRegW),     (NoOP, NoA),       (NoOP, NoA),     (Sty, ZpxRegW),  (Sta, ZpxRegW),     (Stx, ZpyRegW),  (Sax, ZpyRegW)),
            0x98 => low((TStatus, Imp(ReadType::ReadY, WriteType::WriteA)),(Sta, AbyRegWDelay),(TNoStatus, Imp(ReadType::ReadX, WriteType::WriteS)),(NoOP, NoA),     (NoOP, NoA),     (Sta, AbxRegWDelay),(NoOP, NoA),     (NoOP, NoA)),
            0xA0 => low((Ldy, ImmR),     (Lda, IzxR),        (Ldx, ImmR),       (NoOP, NoA),     (Ldy, ZpR),      (Lda, ZpR),         (Ldx, ZpR),      (NoOP, NoA)),
            0xA8 => low((TStatus, Imp(ReadType::ReadA, WriteType::WriteY)),(Lda, ImmR),        (TStatus, Imp(ReadType::ReadA, WriteType::WriteX)),  (NoOP, NoA),     (Ldy, AbsR),     (Lda, AbsR),        (Ldx, AbsR),     (NoOP, NoA)),
            0xB0 => low((Bcs, None),     (Lda, IzyDelayR),   (NoOP, NoA),       (NoOP, NoA),     (Ldy, ZpxR),     (Lda, ZpxR),        (Ldx, ZpyR),     (NoOP, NoA)),
            0xB8 => low((Clv, Imp(ReadType::ReadNull, WriteType::WriteNull)),  (Lda, AbyDelayR),   (TNoStatus, Imp(ReadType::ReadS, WriteType::WriteX)),(NoOP, NoA),     (Ldy, AbxDelayR),(Lda, AbxDelayR),   (Ldx, AbyDelayR),(NoOP, NoA)),
            0xC0 => low((Cpy, ImmR),     (Cmp, IzxR),        (NoOP, NoA),       (Dcp, IzxR),     (Cpy, ZpR),      (Cmp, ZpR),         (Dec, ZpW),      (Dcp, ZpR)),
            0xC8 => low((Inc, Imp(ReadType::ReadY, WriteType::WriteY)),    (Cmp, ImmR),        (Dec, Imp(ReadType::ReadX, WriteType::WriteX)),      (NoOP, NoA),     (Cpy, AbsR),     (Cmp, AbsR),        (Dec, AbsW),     (Dcp, AbsR)),
            0xD0 => low((Bne, None),     (Cmp, IzyDelayR),   (NoOP, NoA),       (Dcp, IzyDelayR),(NoOP, NoA),     (Cmp, ZpxR),        (Dec, ZpxW),     (Dcp, ZpxR)),
            0xD8 => low((Cld, Imp(ReadType::ReadNull, WriteType::WriteNull)),  (Cmp, AbyDelayR),   (NoOP, NoA),       (Dcp, AbyDelayR),(NoOP, NoA),     (Cmp, AbxDelayR),   (Dec, AbxDelayW),(Dcp, AbxDelayR)),
            0xE0 => low((Cpx, ImmR),     (Sbc, IzxR),        (NoOP, NoA),       (NoOP, NoA),     (Cpx, ZpR),      (Sbc, ZpR),         (Inc, ZpW),      (NoOP, NoA)),
            0xE8 => low((Inc, Imp(ReadType::ReadX, WriteType::WriteX)),    (Sbc, ImmR),        (Nop, Imp(ReadType::ReadA, WriteType::WriteA)),      (NoOP, NoA),     (Cpx, AbsR),     (Sbc, AbsR),        (Inc, AbsW),     (NoOP, NoA)),
            0xF0 => low((Beo, None),     (Sbc, IzyDelayR),   (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Sbc, ZpxR),        (Inc, ZpxW),     (NoOP, NoA)),
            0xF8 => low((Sed, Imp(ReadType::ReadNull, WriteType::WriteNull)),  (Sbc, AbyDelayR),   (NoOP, NoA),       (NoOP, NoA),     (NoOP, NoA),     (Sbc, AbxDelayR),   (Inc, AbxDelayW),(NoOP, NoA)),

            _ => {
                panic!("Ocode not implemented: 0x{:x}", op_code);
            }
        }
    }
}

#[cfg(test)]
mod tests {
}
