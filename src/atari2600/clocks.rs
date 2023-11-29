pub type ClockType = u64;

//#[derive(Copy)]
pub struct Clock {
    pub ticks: ClockType,
}

impl Clock {
    pub fn new() -> Self {
        Self { ticks: 0 }
    }

    pub fn increment(&mut self, inc: u32) {
        self.ticks = self.ticks.wrapping_add(inc as u64);
    }
}
