use crate::adq::ADQ;

use super::{ControlRated, Disadvantage};

pub struct RacialIntolerance;

impl RacialIntolerance {
    pub fn new() -> impl ADQ {
        RacialIntolerance
    }
}

impl Disadvantage for RacialIntolerance {}

impl ADQ for RacialIntolerance {
    fn cost(&self) -> f64 {
        -10.0
    }

    fn name(&self) -> &str {
        "Racial Intolerance"
    }
}

impl ControlRated for RacialIntolerance {
    fn control(&self) -> i32 {
        12
    }
}
