use super::inputs;

pub struct Ports {
    pub joysticks: inputs::Joystick,
}

impl Ports {

    pub fn new() -> Self {
        Self {
            joysticks: inputs::Joystick::new(),
        }
    }
}
