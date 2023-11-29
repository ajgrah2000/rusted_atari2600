use super::cartridge;
use super::super::clocks;
use super::super::io;

pub struct Memory {
    cartridge: Box<dyn cartridge::Cartridge>,
    stella: Box<dyn io::ReadWriteMemory>,
    riot: Box<dyn io::ReadWriteMemory>,
}

impl Memory {
    const STELLA_MASK:u16  = 0xFE80;
    const STELLA_ADDR:u16  = 0x0;
    const STACK_OFFSET:u16 = 0x100;
    const STACK_LENGTH:u16 = 0x100;
    const RIOT_MASK:u16    = 0xDC80;
    const RIOT_ADDR:u16    = 0x80;
    const ROM_MASK:u16     = 0xD000;
    const ROM_ADDRLINE:u16 = 0x1000;

    pub fn new(cartridge: Box<dyn cartridge::Cartridge>, stella: Box<dyn io::ReadWriteMemory>, riot: Box<dyn io::ReadWriteMemory>) -> Self{
        Self{
            cartridge: cartridge,
            stella: stella,
            riot: riot,
        }
    }

    pub fn write(&mut self, clock: &clocks::Clock, address: u16, data: u8) {
        if ((address & 0xFFEF) & Memory::STELLA_MASK) == Memory::STELLA_ADDR {
            self.stella.write(clock, address & !Memory::STELLA_MASK, data);
        } else if (address & Memory::RIOT_MASK) == Memory::RIOT_ADDR {
            self.riot.write(clock, address & !Memory::RIOT_MASK, data);
        } else if (address >= Memory::STACK_OFFSET) && ((address < Memory::STACK_OFFSET + Memory::STACK_LENGTH)) {
            self.riot.write(clock, address, data);
        } else if (address & Memory::ROM_ADDRLINE) == Memory::ROM_ADDRLINE {
            // Only address lines 1-13 are connected, higher bits ignored.
            return self.cartridge.write(address & !Memory::ROM_MASK, data);
        }
        else {
            println!("Write: {:#X}", address);
            panic!("invalid_write_address {}", address);
        }
    }

    pub fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
      // Only address lines 1-13 are connected, higher bits ignored.
      if (address & Memory::ROM_ADDRLINE) == Memory::ROM_ADDRLINE {
          return self.cartridge.read(address & !Memory::ROM_MASK);
      }

      if (address & Memory::RIOT_MASK) == Memory::RIOT_ADDR {
          return self.riot.read(clock, address & !Memory::RIOT_MASK);
      }

      if (address & Memory::STELLA_MASK) == Memory::STELLA_ADDR {
          return self.stella.read(clock, address & Memory::STELLA_MASK);
      }

      if (address >= Memory::STACK_OFFSET) && (address < Memory::STACK_OFFSET + Memory::STACK_LENGTH) {
          return self.riot.read(clock, address);
      }

      self.cartridge.read(address & Memory::ROM_MASK)
    }

    pub fn read16(&mut self, clock: &clocks::Clock, address:u16) -> u16 {
        self.read(clock, address) as u16 + ((self.read(clock, address + 1) as u16) << 8)
    }

    pub fn read_sp(&mut self, clock: &clocks::Clock, address:u8) -> u8 {
        self.read(clock, address as u16 + Memory::STACK_LENGTH)
    }

    pub fn write_sp(&mut self, clock: &clocks::Clock, address:u8, data:u8) {
        self.write(clock, address as u16 + Memory::STACK_LENGTH, data);
    }

}
