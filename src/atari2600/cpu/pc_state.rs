use bitfield::bitfield;
use std::fmt;

type Reg8 = u8;
type Reg16 = u16;

bitfield! {
    pub struct PcStatusFlagFields(u8);

    pub get_c,  set_c:  0,0;
    pub get_z,  set_z:  1,1;
    pub get_i,  set_i:  2,2;
    pub get_d,  set_d:  3,3;
    pub get_b,  set_b:  4,4;
    pub get_x1, set_x1: 5,5;
    pub get_v,  set_v:  6,6;
    pub get_n,  set_n:  7,7;
}

impl fmt::Display for PcStatusFlagFields {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        write!(
            dest,
            "(C:{} Z:{} I:{} D:{} B:{} X1:{} V:{} N:{})",
            self.get_c(),
            self.get_z(),
            self.get_i(),
            self.get_d(),
            self.get_b(),
            self.get_x1(),
            self.get_v(),
            self.get_n()
        )
    }
}

pub struct PcState {
    // Registers
    pub a_reg: Reg8,
    pub x_reg: Reg8,
    pub y_reg: Reg8,
    pub pc_reg: Reg16,

    pub s_reg: Reg8,
    pub p_reg: PcStatusFlagFields,
}

impl fmt::Display for PcState {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        write!(dest, "PC:{:X} X:{:X} Y:{:X} A:{:X} {}", self.get_pc(), self.get_x(), self.get_y(), self.get_a(), self.get_flags())
    }
}

impl PcState {
    pub const CYCLES_TO_CLOCK: u8 = 3;

    pub fn new() -> Self {
        Self {
            a_reg: 0,
            x_reg: 0,
            y_reg: 0,
            pc_reg: 0,
            s_reg: 0,
            p_reg: PcStatusFlagFields(0),
        }
    }

    pub fn get_a(&self) -> u8 {
        self.a_reg
    }
    pub fn get_x(&self) -> u8 {
        self.x_reg
    }
    pub fn get_y(&self) -> u8 {
        self.y_reg
    }

    pub fn get_pc(&self) -> u16 {
        self.pc_reg
    }

    pub fn get_pch(&self) -> u8 {
        (self.get_pc() >> 8) as u8
    }

    pub fn get_pcl(&self) -> u8 {
        (self.get_pc() & 0xFF) as u8
    }

    pub fn get_s(&self) -> u8 {
        self.s_reg
    }

    pub fn get_p(&self) -> u8 {
        self.p_reg.0
    }

    pub fn get_flags(&self) -> PcStatusFlagFields {
        PcStatusFlagFields(self.p_reg.0)
    }

    pub fn get_flag_c(&self) -> bool {
        self.p_reg.get_c() != 0
    }

    pub fn get_flag_z(&self) -> bool {
        self.p_reg.get_z() != 0
    }

    pub fn get_flag_i(&self) -> bool {
        self.p_reg.get_i() != 0
    }

    pub fn get_flag_d(&self) -> bool {
        self.p_reg.get_d() != 0
    }

    pub fn get_flag_b(&self) -> bool {
        self.p_reg.get_b() != 0
    }

    pub fn get_flag_x1(&self) -> bool {
        self.p_reg.get_x1() != 0
    }

    pub fn get_flag_v(&self) -> bool {
        self.p_reg.get_v() != 0
    }

    pub fn get_flag_n(&self) -> bool {
        self.p_reg.get_n() != 0
    }

    pub fn set_flag_c(&mut self, value: bool) {
        self.p_reg.set_c(value as u8);
    }

    pub fn set_flag_z(&mut self, value: bool) {
        self.p_reg.set_z(value as u8);
    }

    pub fn set_flag_i(&mut self, value: bool) {
        self.p_reg.set_i(value as u8);
    }

    pub fn set_flag_d(&mut self, value: bool) {
        self.p_reg.set_d(value as u8);
    }

    pub fn set_flag_b(&mut self, value: bool) {
        self.p_reg.set_b(value as u8);
    }

    pub fn set_flag_x1(&mut self, value: bool) {
        self.p_reg.set_x1(value as u8);
    }

    pub fn set_flag_v(&mut self, value: bool) {
        self.p_reg.set_v(value as u8);
    }

    pub fn set_flag_n(&mut self, value: bool) {
        self.p_reg.set_n(value as u8);
    }

    pub fn set_a(&mut self, input: u8) {
        self.a_reg = input;
    }
    pub fn set_x(&mut self, input: u8) {
        self.x_reg = input;
    }
    pub fn set_y(&mut self, input: u8) {
        self.y_reg = input;
    }

    pub fn set_pc(&mut self, input: u16) {
        self.pc_reg = input;
    }

    pub fn set_pch(&mut self, input: u8) {
        self.pc_reg = self.pc_reg & 0xFF | (input as u16) << 8;
    }

    pub fn set_pcl(&mut self, input: u8) {
        self.pc_reg = self.pc_reg & 0xFF00 | input as u16;
    }

    pub fn set_s(&mut self, input: u8) {
        self.s_reg = input;
    }

    pub fn set_p(&mut self, input: u8) {
        self.p_reg.0 = input;
    }

    pub fn increment_reg8(register: &mut Reg8, increment: i8) {
        *register = (*register as i8).wrapping_add(increment) as u8;
    }

    pub fn increment_reg(register: &mut Reg16, increment: i16) {
        *register = (*register as i16).wrapping_add(increment) as u16;
    }

    pub fn increment_s(&mut self, increment: i8) {
        Self::increment_reg8(&mut self.s_reg, increment);
    }

    pub fn increment_pc(&mut self, increment: i16) {
        Self::increment_reg(&mut self.pc_reg, increment);
    }
}

pub fn set_status_nz(pc_state: &mut PcState, value: u8) {
    pc_state.set_flag_n(0x80 == 0x80 & value);
    pc_state.set_flag_z(0x00 == value);
}

type ReadFunc = fn(&PcState) -> u8; // Unused: Switch to 'trait alias' when available

pub fn read_null(pc_state: &PcState) -> u8 {
    0
}
pub fn read_x(pc_state: &PcState) -> u8 {
    pc_state.get_x()
}
pub fn read_y(pc_state: &PcState) -> u8 {
    pc_state.get_y()
}
pub fn read_a(pc_state: &PcState) -> u8 {
    pc_state.get_a()
}
pub fn read_s(pc_state: &PcState) -> u8 {
    pc_state.get_s()
}

type WriteFunc = fn(&mut PcState, u8); // Unused: Switch to 'trait alias' when available

pub fn write_null(pc_state: &mut PcState, input: u8) {}
pub fn write_x(pc_state: &mut PcState, input: u8) {
    pc_state.set_x(input);
}
pub fn write_y(pc_state: &mut PcState, input: u8) {
    pc_state.set_y(input);
}
pub fn write_a(pc_state: &mut PcState, input: u8) {
    pc_state.set_a(input);
}
pub fn write_s(pc_state: &mut PcState, input: u8) {
    pc_state.set_s(input);
}

#[cfg(test)]
mod tests {
    use crate::atari2600::cpu::pc_state::PcState;
    #[test]
    fn test_display_pc_state() {
        let mut pc_state = PcState::new();
        assert_eq!(format!("{}", pc_state), "PC:0 X:0 Y:0 A:0 (C:0 Z:0 I:0 D:0 B:0 X1:0 V:0 N:0)");

        pc_state.set_flag_x1(true);

        // Use the formatted state to check the output.
        assert_eq!(format!("{}", pc_state), "PC:0 X:0 Y:0 A:0 (C:0 Z:0 I:0 D:0 B:0 X1:1 V:0 N:0)");
    }
}
