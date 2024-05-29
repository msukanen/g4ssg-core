use crate::life::adq::{ControlRated, ADQ};

use super::Disadvantage;

pub struct Overconfidence;

impl Disadvantage for Overconfidence {
    fn is_overconfidence(&self) -> bool {
        true
    }
}

impl ControlRated for Overconfidence {
    fn control(&self) -> i32 {
        12
    }
}

impl ADQ for Overconfidence {
    fn cost(&self) -> f64 {
        -10.0
    }

    fn name(&self) -> &str {
        "Overconfidence"
    }
}
