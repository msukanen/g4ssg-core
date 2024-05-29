use crate::life::adq::{cost_from_control, ControlRated, ADQ};

use super::Advantage;

pub struct Selfless {
    control: i32,
    name: String,
}

impl Selfless {
    pub fn new(control: i32) -> Selfless {
        Selfless {
            name: format!("Selfless ({control})"),
            control
        }
    }
}

impl Advantage for Selfless {}
impl ADQ for Selfless {
    fn cost(&self) -> f64 {
        cost_from_control(self)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
impl ControlRated for Selfless {
    fn control(&self) -> i32 {
        self.control
    }
}
