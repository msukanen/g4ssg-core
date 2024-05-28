use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Undiscriminating;

impl Quirk for Undiscriminating {}
impl Disadvantage for Undiscriminating {}
impl ADQ for Undiscriminating {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Undiscriminating"
    }
}
impl ControlRated for Undiscriminating {}
