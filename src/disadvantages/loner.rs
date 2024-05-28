use crate::adq::{cost_from_control, normalize_control, ControlRated, ADQ};

use super::Disadvantage;

pub struct Loner {
    control: i32,
    name: String,
}

impl Loner {
    pub fn new(mut control: i32) -> Loner {
        control = normalize_control(control);
        Loner {
            name: format!("Loner ({control}"),
            control
        }
    }
}

impl Disadvantage for Loner {}
impl ControlRated for Loner {
    fn control(&self) -> i32 {
        self.control
    }
}
impl ADQ for Loner {
    fn cost(&self) -> f64 {
        -cost_from_control(self)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
