use crate::adq::ADQ;

use super::{cost_from_control, normalize_control, ControlRated, Disadvantage};

pub struct Incurious {
    control: i32,
    name: String,
}

impl Incurious {
    pub fn new(mut control: i32) -> Incurious {
        control = normalize_control(control);
        Incurious { name: format!("Incurious ({control})"), control }
    }
}

impl Disadvantage for Incurious {}
impl ADQ for Incurious {
    fn cost(&self) -> f64 {
        -cost_from_control(self)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl ControlRated for Incurious {
    fn control(&self) -> i32 {
        self.control
    }
}
