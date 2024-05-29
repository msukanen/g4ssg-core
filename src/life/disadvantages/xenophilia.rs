use crate::life::adq::ADQ;

use super::{ControlRated, Disadvantage};

pub struct Xenophilia {
    control: i32,
}

impl Disadvantage for Xenophilia {}
impl ADQ for Xenophilia {
    fn cost(&self) -> f64 {
        -15.0
    }

    fn name(&self) -> &str {
        "Xenophilia"
    }
}

impl ControlRated for Xenophilia {
    fn control(&self) -> i32 {
        self.control
    }
}

impl Xenophilia {
    pub fn new(control: i32) -> Xenophilia {
        Xenophilia { control }
    }
}
