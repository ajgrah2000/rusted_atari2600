use bitfield::bitfield;
use std::fmt;

type Reg8  = u8;
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
    pub a_reg:  Reg8,
    pub x_reg:  Reg8,
    pub y_reg:  Reg8,
    pub pc_reg: Reg16,

    pub s_reg:  Reg8,
    pub p_reg:  Reg8,
}

impl fmt::Display for PcState {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        write!(dest, "PC:{} X:{} Y:{} A:{} {}",
                self.get_pc(), self.get_x(), self.get_y(), 
                self.get_a(), self.get_flags())
    }
}


impl PcState {
    pub fn new() -> Self {
        Self {
            a_reg:  0,
            x_reg:  0,
            y_reg:  0,
            pc_reg: 0,
            s_reg:  0,
            p_reg:  0,
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

    pub fn get_s(&self) -> u8 {
        self.s_reg
    }

    pub fn get_p(&self) -> u8 {
        self.p_reg
    }

    pub fn get_flags(&self) -> PcStatusFlagFields {
        PcStatusFlagFields(self.p_reg)
    }

    pub fn set_a(&mut self, input: u8) {
        self.a_reg = input;
    }
    pub fn set_x(&mut self, input: u8) {
        self.x_reg = input;
    }
    pub fn set_y(&mut self, input:u8) {
        self.y_reg = input;
    }

    pub fn set_pc(&mut self, input:u16) {
        self.pc_reg = input;
    }

    pub fn set_s(&mut self, input:u8) {
        self.s_reg = input;
    }

    pub fn set_flags(&mut self, input: PcStatusFlagFields) {
        self.p_reg = input.0;
    }

    pub fn set_p(&mut self, input: u8) {
        self.p_reg = input;
    }
}


#[cfg(test)]
mod tests {
    use crate::atari2600::cpu::pc_state::PcState;
    #[test]
    fn test_display_pc_state() {
        let mut pc_state = PcState::new();
        assert_eq!(format!("{}", pc_state), "PC:0 X:0 Y:0 A:0 (C:0 Z:0 I:0 D:0 B:0 X1:0 V:0 N:0)");

        let mut pc_state_flags = pc_state.get_flags();

        pc_state_flags.set_x1(1); 

        pc_state.set_flags(pc_state_flags); 

        // Use the formatted state to check the output.
        assert_eq!(format!("{}", pc_state), "PC:0 X:0 Y:0 A:0 (C:0 Z:0 I:0 D:0 B:0 X1:1 V:0 N:0)");
    }
}
