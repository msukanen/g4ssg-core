use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Staid;

impl Quirk for Staid {}
impl Disadvantage for Staid {}
impl ADQ for Staid {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Staid"
    }
}
impl ControlRated for Staid {}
