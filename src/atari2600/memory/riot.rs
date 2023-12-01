use super::super::clocks;
use super::super::io;
use super::super::inputs;

#[derive(Clone,Copy)]
enum Interval {
    Tim1 = 1,
    Tim8 = 8,
    Tim64 = 64,
    Tim1024 = 1024,
}

pub struct Riot {
//inputs:
    input:inputs::Input,   // Interval holds the values
    interval:Interval,   // Interval holds the values
    expiration_time: clocks::ClockType,
    ram: Vec<u8>,
}

type AddressType = u16;

impl Riot {
    const CYCLES_TO_CLOCK:clocks::ClockType = 3;
    const RAMSIZE:u8         = 128;
    const NOT_RAMSELECT:AddressType  = 0x200;
    const RIOT_ADDRMASK:u8   = 0x7F;
    const RIOT_SWCHA:u8      = 0x00;
    const RIOT_SWCHB:u8      = 0x02;
    const TIMERADDR:u8       = 0x04;
    const RIOT_INTERRUPT:u8  = 0x05;

    const INT_ENABLE_MASK:u8 = 0x8;

    const RIOT_TIM1T:u8      = 0x14;
    const RIOT_TIM8T:u8      = 0x15;
    const RIOT_TIM64T:u8     = 0x16;
    const RIOT_T1024T:u8     = 0x17;

    pub fn new() -> Self {
        Self {
            input:inputs::Input::new(),
            interval:Interval::Tim1024,
            expiration_time: 1000000,
            ram: vec![0; Riot::RAMSIZE as usize],
        }
    }

    pub fn set_inputs(&mut self, inputs: inputs::Input) {
        // TODO: Call this function from somewhere.
        self.input = inputs;
    }

    pub fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        let mut value:u8 = 0;

        let future_clock = clock.ticks + 12;

        if 0 == (address & Riot::NOT_RAMSELECT) {
            return self.ram[(address as u8 & Riot::RIOT_ADDRMASK) as usize];
        }

        // Ignore interrupt enable address line.
        let test = (address as u8) & Riot::RIOT_ADDRMASK & !Riot::INT_ENABLE_MASK;

        if test == Riot::RIOT_SWCHA {
            value = self.input.swcha;
        } else if test == Riot::RIOT_SWCHB {
            value = self.input.swchb;
        }
        else if test == Riot::RIOT_TIM1T || test == Riot::RIOT_TIM8T || test == Riot::RIOT_TIM64T || test == Riot::RIOT_T1024T || test == Riot::TIMERADDR
        {
            if self.expiration_time >= future_clock {
                // If expiration hasn't occured, return the time remaining.
                value = ((self.expiration_time - future_clock) / (self.interval as clocks::ClockType * Riot::CYCLES_TO_CLOCK)) as u8;
            } else {
                // Calculate ticks past zero, may not be quite right
                // The interval was passed, value counts down from 255.
                value = (0x100 as i16).wrapping_sub(((future_clock - self.expiration_time)/Riot::CYCLES_TO_CLOCK) as i16) as u8;
            }
        }
        else if test == Riot::RIOT_INTERRUPT {
            if self.expiration_time >= future_clock {
                value = 0;
            } else {
                // Return the interrupt flag if time has expired.
                value = 0x80;
            }
        }
        else
        {
            println!("Bad address: {:X}", address);
        }

        value
    }

    pub fn write(&mut self, clock: &clocks::Clock, address: u16, data: u8) {
        if 0 == (address & Riot::NOT_RAMSELECT) {
            self.ram[(address as u8 & Riot::RIOT_ADDRMASK) as usize] = data;
        } else {
            let test = address as u8 & Riot::RIOT_ADDRMASK;
            if test == Riot::RIOT_TIM1T {
                self.interval = Interval::Tim1;
            }
            else if test == Riot::RIOT_TIM8T {
                self.interval = Interval::Tim8;
            } else if test == Riot::RIOT_TIM64T {
                self.interval = Interval::Tim64;
            } else if test == Riot::RIOT_T1024T {
                self.interval = Interval::Tim1024;
            } else {
                println!("Nothing written: {:X}", address);
            }

            self.expiration_time = clock.ticks + Riot::CYCLES_TO_CLOCK * data  as clocks::ClockType * self.interval as clocks::ClockType;
        }
    }

}

impl io::ReadWriteMemory for Riot {
    fn write(&mut self, clock: &mut clocks::Clock, address: u16, data: u8) {
        self.write(clock, address, data);
    }
    fn read(&mut self, clock: &clocks::Clock, address: u16) -> u8 {
        self.read(clock, address)
    }
}
