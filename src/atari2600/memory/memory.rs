use super::cartridge;

pub struct Memory {
    cartridge: Box<dyn cartridge::Cartridge>,
}

impl Memory {
    pub fn new(cartridge: Box<dyn cartridge::Cartridge>) -> Self{
        Self{
            cartridge: cartridge,
        }
    }

}
