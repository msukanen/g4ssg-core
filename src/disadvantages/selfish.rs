use crate::adq::{cost_from_control, normalize_control, ADQ};

use super::{ControlRated, Disadvantage};

pub struct Selfish {
    control: i32,
    name: String,
}

impl Selfish {
    pub fn new(mut control: i32) -> Selfish {
        control = normalize_control(control);
        Selfish {
            name: format!("Selfish ({control}"),
            control
        }
    }
}

impl Disadvantage for Selfish {}
impl ADQ for Selfish {
    fn cost(&self) -> f64 {
        -cost_from_control(self)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl ControlRated for Selfish {
    fn control(&self) -> i32 {
        self.control
    }
}
