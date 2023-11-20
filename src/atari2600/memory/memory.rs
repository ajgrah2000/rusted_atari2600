use super::cartridge;

pub struct Memory {
}

impl Memory {
    pub fn new() -> Self{
        Self{
        }
    }
    pub fn set_cartridge(&mut self, cartridge: &dyn cartridge::Cartridge) {
        // TODO
    }
}
