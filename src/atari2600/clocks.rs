pub type ClockType = u64;

//#[derive(Copy)]
pub struct Clock {
    pub cycles: ClockType,
}

impl Clock {
    pub fn new() -> Self {
        Self { cycles: 0 }
    }

    pub fn increment(&mut self, inc: u32) {
        self.cycles = self.cycles.wrapping_add(inc as u64);
    }
}
