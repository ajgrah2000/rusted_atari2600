use super::clocks;

pub struct Core {
}

impl Core {
  pub fn new() -> Self {
    Core {}
  }
}

pub struct Atari2600 {
    core: Core,
    debug: bool,
    realtime: bool,
    stop_clock: clocks::ClockType,
    fullscreen: bool,
}

impl Atari2600 {
    pub fn build_atari2600(cartridge_name: String) -> Core {
      Core::new()
    }

    pub fn power_atari2600(&mut self) {
    }

    pub fn new(debug: bool, realtime: bool, stop_clock:clocks::ClockType, cartridge_name: String, fullscreen: bool) -> Self {
    
        let core = Self::build_atari2600(cartridge_name);
        Self { core, debug, realtime, stop_clock, fullscreen }
    }
}
