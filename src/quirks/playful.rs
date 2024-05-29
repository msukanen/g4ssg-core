use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Playful;

impl Quirk for Playful {}
impl Disadvantage for Playful {}
impl ControlRated for Playful {}
impl ADQ for Playful {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Playful"
    }
}
