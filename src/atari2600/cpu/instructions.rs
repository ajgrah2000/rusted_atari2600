use super::super::clocks;
use super::super::memory::addressing;
use super::super::memory::addressing::Addressing;
use super::super::memory::memory;
use super::super::ports;
use super::instruction_set;
use super::pc_state;

pub struct Instruction {}

// Page Delay version of addressing modes (only applicable to some indexed modes, that can carry).)
const NULL_READ: addressing::NullRead = addressing::NullRead::new();
const MEMORY_READ: addressing::MemoryRead = addressing::MemoryRead::new();
const ACCUMULATOR_READ: addressing::AccumulatorRead = addressing::AccumulatorRead::new();
const MEMORY_WRITE: addressing::MemoryWrite = addressing::MemoryWrite::new();
const ACCUMULATOR_WRITE: addressing::AccumulatorWrite = addressing::AccumulatorWrite::new();
const REG_WRITE: addressing::RegisterWrite = addressing::RegisterWrite::new();
const MEMORY_NULL: addressing::MemoryNull = addressing::MemoryNull::new();

use AddressMode::*;
use OpName::*;
use RegisterName::*;

#[derive(Debug, Copy, Clone)]
enum RegisterName {
    N,
    X,
    Y,
    A,
    S,
} // 'N - Null/No register

#[derive(Debug, Copy, Clone)]
enum OpName {
    Adc,
    And,
    Asl,
    Bit,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dcp,
    Dec,
    Eor,
    Inc,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Or,
    Rol,
    Ror,
    Sax,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    TNoStat,
    TStat,
    Jsr,
    Brk,
    Rti,
    Rts,
    JmpAbs,
    JmpInd,
    Php,
    Plp,
    Pha,
    Pla,
    Br(u8, bool),
    Isc,
    Slo,
    Lax, // Undocumented op-codes
    NoOP,
}

#[derive(Debug, Copy, Clone)]
enum AddressMode {
    Imp(RegisterName, RegisterName), // (Read, Write)
    IzxR,
    IzyR,
    ImmR,
    ZpR,
    ZpyR,
    ZpxR,
    IzyRD,
    AbsR,
    AbxR,
    AbyR,
    AbxRD,
    AbyRD,
    IzxW,
    IzyW,
    ZpW,
    ZpxW,
    AbsW,
    AbxW,
    AbxWD,
    AbyW,
    Acc,
    IzxRegW,
    ZpRegW,
    ZpxRegW,
    ZpyRegW,
    IzyRegW,
    AbsRegW,
    None,
    AbxRegWD,
    AbyRegWD,
    NoA,
}

impl Instruction {
    pub fn execute(op_code: u8, clock: &mut clocks::Clock, memory: &mut memory::Memory, pc_state: &mut pc_state::PcState, ports: &mut ports::Ports) {
        use instruction_set::*;

        let op_fn = |op| match op {
            Adc => adc,
            And => and,
            Asl => asl,
            Bit => bit,
            Clc => clc,
            Cld => cld,
            Cli => cli,
            Clv => clv,
            Cmp => cmp,
            Cpx => cpx,
            Cpy => cpy,
            Dcp => dcp,
            Dec => dec,
            Eor => eor,
            Inc => inc,
            Isc => isc,
            Lax => lax,
            Lda => lda,
            Ldx => ldx,
            Ldy => ldy,
            Lsr => lsr,
            Nop => nop,
            Or => or,
            Rol => rol,
            Ror => ror,
            Sax => sax,
            Sbc => sbc,
            Sec => sec,
            Sed => sed,
            Sei => sei,
            Slo => slo,
            Sta => sta,
            Stx => stx,
            Sty => sty,
            TNoStat => t_no_status,
            TStat => t_status,
            _ => {
                panic!("Unexpected operator {:?}", op);
            }
        };

        let addressing_fn = |addr| match addr {
            IzxR | IzxRegW | IzxW => &Addressing::Izx,
            IzyR | IzyRegW | IzyW => &Addressing::Izy,
            ImmR => &Addressing::Imm,
            ZpR | ZpW | ZpRegW => &Addressing::Zp,
            ZpxR | ZpxW | ZpxRegW => &Addressing::Zpx,
            ZpyR | ZpyRegW => &Addressing::Zpy,
            IzyRD => &Addressing::IZYPageDelay,
            AbsR | AbsW | AbsRegW => &Addressing::Abs,
            AbxR | AbxW | AbyW => &Addressing::Abx,
            AbyR => &Addressing::Aby,
            AbxRD | AbxWD => &Addressing::AbxPageDelay,
            AbyRD => &Addressing::AbyPageDelay,
            Acc => &Addressing::Accumulator,

            _ => {
                panic!("Unexpected addressing mode {:?}", addr);
            }
        };

        let read_fn = |read_type| match read_type {
            N => pc_state::read_null,
            X => pc_state::read_x,
            Y => pc_state::read_y,
            A => pc_state::read_a,
            S => pc_state::read_s,
        };

        let write_fn = |write_type| match write_type {
            N => pc_state::write_null,
            X => pc_state::write_x,
            Y => pc_state::write_y,
            A => pc_state::write_a,
            S => pc_state::write_s,
        };

        let mut op = |op_arg, addr| {
            match (addr, op_arg) {
                (Imp(r, w), o) => Ok(instruction_set::single_byte_instruction(clock, pc_state, memory, read_fn(r), write_fn(w), op_fn(o))),

                (read_null @ (IzxR | IzyR | ImmR | ZpR | ZpxR | ZpyR | IzyRD | AbsR | AbxR | AbyR | AbxRD | AbyRD), o) => {
                    Ok(instruction_set::read_write_instruction(clock, pc_state, memory, addressing_fn(read_null), MEMORY_READ, MEMORY_NULL, op_fn(o)))
                }
                (read_write @ (IzxW | IzyW | ZpW | ZpxW | AbsW | AbxWD | AbxW | AbyW), o) => Ok(instruction_set::read_write_instruction(clock, pc_state, memory, addressing_fn(read_write), MEMORY_READ, MEMORY_WRITE, op_fn(o))),
                (Acc, o) => Ok(instruction_set::read_write_instruction(clock, pc_state, memory, &Addressing::Accumulator, ACCUMULATOR_READ, ACCUMULATOR_WRITE, op_fn(o))),
                (reg_write @ (IzxRegW | ZpRegW | ZpxRegW | IzyRegW | AbsRegW | ZpyRegW), o) => Ok(instruction_set::read_write_instruction(clock, pc_state, memory, addressing_fn(reg_write), NULL_READ, REG_WRITE, op_fn(o))),

                (AbxRegWD, o) => Ok(instruction_set::read_write_instruction_additional_delay(
                    clock,
                    pc_state,
                    memory,
                    &Addressing::Abx,
                    NULL_READ,
                    REG_WRITE,
                    op_fn(o),
                    pc_state::PcState::CYCLES_TO_CLOCK,
                )),
                (AbyRegWD, o) => Ok(instruction_set::read_write_instruction_additional_delay(
                    clock,
                    pc_state,
                    memory,
                    &Addressing::Aby,
                    NULL_READ,
                    REG_WRITE,
                    op_fn(o),
                    pc_state::PcState::CYCLES_TO_CLOCK,
                )),

                (None, Jsr) => Ok(instruction_set::jump_sub_routine_instruction(clock, pc_state, memory)),
                (None, Brk) => Ok(instruction_set::break_instruction(clock, pc_state, memory)),
                (None, Rti) => Ok(instruction_set::return_from_interrupt(clock, pc_state, memory)),
                (None, Rts) => Ok(instruction_set::return_from_sub_routine_instruction(clock, pc_state, memory)),
                (None, JmpAbs) => Ok(instruction_set::jump_instruction(clock, pc_state, memory, &Addressing::Abs)),
                (None, JmpInd) => Ok(instruction_set::jump_instruction(clock, pc_state, memory, &Addressing::Indirect)),
                (None, Php) => Ok(instruction_set::php_instruction(clock, pc_state, memory)),
                (None, Plp) => Ok(instruction_set::plp_instruction(clock, pc_state, memory)),
                (None, Pha) => Ok(instruction_set::pha_instruction(clock, pc_state, memory)),
                (None, Pla) => Ok(instruction_set::pla_instruction(clock, pc_state, memory)),

                (None, Br(m, v)) => Ok(instruction_set::branch_instruction(clock, pc_state, memory, 1 << m, (1 << m) * (v as u8))), // N == 1
                _ => Err(format!("Unexpected address operator combination {:?} {:?}", addr, op_arg)),
            }
        };

        // Mnemonic simplifications
        const NA: (OpName, AddressMode) = (NoOP, NoA); // Not applicable/operation not implemented.

        // Bpl: 0x80, 0x00, N == 0, Bmi: 0x80, 0x80, N == 1, Bvc: 0x40, 0x00, V == 0, Bvs: 0x40, 0x40, V == 1,
        // Bcc: 0x01, 0x00, C == 0, Bcs: 0x01, 0x01, C == 1, Bne: 0x02, 0x00, Z == 0, Beo: 0x02, 0x02, Z == 1,
        let (bpl, bmi, bvc, bvs, bcc, bcs, bne, beo) = (
            (Br(7, false), None),
            (Br(7, true), None),
            (Br(6, false), None),
            (Br(6, true), None),
            (Br(0, false), None),
            (Br(0, true), None),
            (Br(1, false), None),
            (Br(1, true), None),
        );

        let mut low = |(op0, a0), (op1, a1), (op2, a2), (op3, a3), (op4, a4), (op5, a5), (op6, a6), (op7, a7)| match op_code & 0x7 {
            0 => op(op0, a0),
            1 => op(op1, a1),
            2 => op(op2, a2),
            3 => op(op3, a3),
            4 => op(op4, a4),
            5 => op(op5, a5),
            6 => op(op6, a6),
            7 => op(op7, a7),
            _ => unreachable!("Me panic? Not Possible."),
        };

        if let Err(err) = match op_code & 0xF8 {
            0x00 => low((Brk, None), (Or, IzxR), NA, (Slo, IzxW), (Nop, ZpR), (Or, ZpR), (Asl, ZpW), (Slo, ZpW)),
            0x08 => low((Php, None), (Or, ImmR), (Asl, Imp(A, A)), NA, (Nop, AbsR), (Or, AbsR), (Asl, AbsW), (Slo, AbsW)),
            0x10 => low(bpl, (Or, IzyRD), NA, (Slo, IzyW), (Nop, ZpxR), (Or, ZpxR), (Asl, ZpxW), (Slo, ZpxW)),
            0x18 => low((Clc, Imp(N, N)), (Or, AbyRD), (Nop, Imp(N, N)), (Slo, AbyW), (Nop, AbxR), (Or, AbxRD), (Asl, AbxWD), (Slo, AbxW)),
            0x20 => low((Jsr, None), (And, IzxR), NA, NA, (Bit, ZpR), (And, ZpR), (Rol, ZpW), NA),
            0x28 => low((Plp, None), (And, ImmR), (Rol, Acc), NA, (Bit, AbsR), (And, AbsR), (Rol, AbsW), NA),
            0x30 => low(bmi, (And, IzyRD), NA, NA, (Nop, ZpxR), (And, ZpxR), (Rol, ZpxW), NA),
            0x38 => low((Sec, Imp(N, N)), (And, AbyRD), (Nop, Imp(N, N)), NA, (Nop, AbxR), (And, AbxRD), (Rol, AbxW), NA),
            0x40 => low((Rti, None), (Eor, IzxR), NA, NA, (Nop, ZpR), (Eor, ZpR), (Lsr, ZpW), NA),
            0x48 => low((Pha, None), (Eor, ImmR), (Lsr, Imp(A, A)), NA, (JmpAbs, None), (Eor, AbsR), (Lsr, AbsW), NA),
            0x50 => low(bvc, (Eor, IzyR), NA, NA, (Nop, ZpxR), (Eor, ZpxR), (Lsr, ZpxW), NA),
            0x58 => low((Cli, Imp(N, N)), (Eor, AbyR), (Nop, Imp(N, N)), NA, (Nop, AbxR), (Eor, AbxR), (Lsr, AbxWD), NA),
            0x60 => low((Rts, None), (Adc, IzxR), NA, NA, (Nop, ZpR), (Adc, ZpR), (Ror, ZpW), NA),
            0x68 => low((Pla, None), (Adc, ImmR), (Ror, Acc), NA, (JmpInd, None), (Adc, AbsR), (Ror, AbsW), NA),
            0x70 => low(bvs, (Adc, IzyRD), NA, NA, (Nop, ZpxR), (Adc, ZpxR), (Ror, ZpxW), NA),
            0x78 => low((Sei, Imp(N, N)), (Adc, AbyRD), (Nop, Imp(N, N)), NA, (Nop, AbxR), (Adc, AbxRD), (Ror, AbxW), NA),
            0x80 => low((Nop, ImmR), (Sta, IzxRegW), (Nop, ImmR), (Sax, IzxRegW), (Sty, ZpRegW), (Sta, ZpRegW), (Stx, ZpRegW), (Sax, ZpRegW)),
            0x88 => low((Dec, Imp(Y, Y)), (Nop, ImmR), (TStat, Imp(X, A)), NA, (Sty, AbsRegW), (Sta, AbsRegW), (Stx, AbsRegW), (Sax, AbsRegW)),
            0x90 => low(bcc, (Sta, IzyRegW), NA, NA, (Sty, ZpxRegW), (Sta, ZpxRegW), (Stx, ZpyRegW), (Sax, ZpyRegW)),
            0x98 => low((TStat, Imp(Y, A)), (Sta, AbyRegWD), (TNoStat, Imp(X, S)), NA, NA, (Sta, AbxRegWD), NA, NA),
            0xA0 => low((Ldy, ImmR), (Lda, IzxR), (Ldx, ImmR), (Lax, IzxR), (Ldy, ZpR), (Lda, ZpR), (Ldx, ZpR), (Lax, ZpR)),
            0xA8 => low((TStat, Imp(A, Y)), (Lda, ImmR), (TStat, Imp(A, X)), NA, (Ldy, AbsR), (Lda, AbsR), (Ldx, AbsR), (Lax, AbsR)),
            0xB0 => low(bcs, (Lda, IzyRD), NA, (Lax, IzyR), (Ldy, ZpxR), (Lda, ZpxR), (Ldx, ZpyR), (Lax, ZpyR)),
            0xB8 => low((Clv, Imp(N, N)), (Lda, AbyRD), (TNoStat, Imp(S, X)), NA, (Ldy, AbxRD), (Lda, AbxRD), (Ldx, AbyRD), (Lax, AbyR)),
            0xC0 => low((Cpy, ImmR), (Cmp, IzxR), (Nop, ImmR), (Dcp, IzxR), (Cpy, ZpR), (Cmp, ZpR), (Dec, ZpW), (Dcp, ZpR)),
            0xC8 => low((Inc, Imp(Y, Y)), (Cmp, ImmR), (Dec, Imp(X, X)), NA, (Cpy, AbsR), (Cmp, AbsR), (Dec, AbsW), (Dcp, AbsR)),
            0xD0 => low(bne, (Cmp, IzyRD), NA, (Dcp, IzyRD), (Nop, ZpxR), (Cmp, ZpxR), (Dec, ZpxW), (Dcp, ZpxR)),
            0xD8 => low((Cld, Imp(N, N)), (Cmp, AbyRD), (Nop, Imp(N, N)), (Dcp, AbyRD), (Nop, AbxR), (Cmp, AbxRD), (Dec, AbxWD), (Dcp, AbxRD)),
            0xE0 => low((Cpx, ImmR), (Sbc, IzxR), (Nop, ImmR), (Isc, IzxW), (Cpx, ZpR), (Sbc, ZpR), (Inc, ZpW), (Isc, ZpW)),
            0xE8 => low((Inc, Imp(X, X)), (Sbc, ImmR), (Nop, Imp(A, A)), NA, (Cpx, AbsR), (Sbc, AbsR), (Inc, AbsW), (Isc, AbsW)),
            0xF0 => low(beo, (Sbc, IzyRD), NA, (Isc, IzyW), (Nop, ZpxR), (Sbc, ZpxR), (Inc, ZpxW), (Isc, ZpxW)),
            0xF8 => low((Sed, Imp(N, N)), (Sbc, AbyRD), (Nop, Imp(N, N)), (Isc, AbyW), (Nop, AbxR), (Sbc, AbxRD), (Inc, AbxWD), (Isc, AbxW)),

            _ => Err(format!("Ocode not implemented: 0x{:x}", op_code)),
        } {
            panic!("Ocode not implemented: 0x{:x}. ({})", op_code, err)
        };
    }
}

#[cfg(test)]
mod tests {}
