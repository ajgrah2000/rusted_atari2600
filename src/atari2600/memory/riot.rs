use super::super::clocks;
use super::super::io;

pub struct Riot {
//inputs:
//interval: 
//expiration_time:
            ram: Vec<u8>,
}


impl Riot {
    const CYCLES_TO_CLOCK:u8 = 3;
    const RAMSIZE:u8         = 128;
    const NOT_RAMSELECT:u16  = 0x200;
    const RIOT_ADDRMASK:u8   = 0x7F;
    const RIOT_Swcha:u8      = 0x00;
    const RIOT_Swchb:u8      = 0x02;
    const TIMERADDR:u8       = 0x04;
    const RIOT_Interrupt:u8  = 0x05;

    const INT_ENABLE_MASK:u8 = 0x8;

    const RIOT_Tim1t:u8      = 0x14;
    const RIOT_Tim8t:u8      = 0x15;
    const RIOT_Tim64t:u8     = 0x16;
    const RIOT_T1024t:u8     = 0x17;

    pub fn new() -> Self {
        Self {
            ram: vec![0; Riot::RAMSIZE as usize],
        }
    }

    pub fn write(&mut self, clock: &clocks::Clock, address: u16, data: u8) {
        // TODO
    }
    
    pub fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
//        let value = 0
//    
//        let future_clock = self.clock.system_clock + 12
//    
//        if 0 == (addr & self.NOT_RAMSELECT):
//            return self.ram[addr & self.RIOT_ADDRMASK]
//    
//        # Ignore interrupt enable address line.
//        test = addr & self.RIOT_ADDRMASK & ~self.INT_ENABLE_MASK
//        if test == self.RIOT_Swcha:
//            value = self.inputs.swcha
//    
//        elif test == self.RIOT_Swchb:
//            value = self.inputs.swchb
//    
//        elif test == self.RIOT_Tim1t or test == self.RIOT_Tim8t or test == self.RIOT_Tim64t or test == self.RIOT_T1024t or test == self.TIMERADDR:
//
//            if self.expiration_time >= future_clock:
//                # If expiration hasn't occured, return the time remaining. 
//                value = (self.expiration_time - future_clock) / (self.interval * self.CYCLES_TO_CLOCK)
//            else: # Calculate ticks past zero, may not be quite right
//                # The interval was passed, value counts down from 255. 
//                value = 0x100 - (((future_clock - self.expiration_time)/self.CYCLES_TO_CLOCK) & 0xFF)
//        elif test == self.RIOT_Interrupt:
//            if self.expiration_time >= future_clock:
//                value = 0
//            else:
//                # Return the interrupt flag if time has expired. 
//                value = 0x80
//        else:
//            print("Bad address:", addr)
//    
//        return value
        0
    }
}

impl io::ReadWriteMemory for Riot {
    fn write(&mut self, clock: &clocks::Clock, address: u16, data: u8) {
        self.write(clock, address, data);
    }
    fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        self.read(clock, address)
    }
}
