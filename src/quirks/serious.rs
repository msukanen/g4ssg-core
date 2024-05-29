use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Serious;

impl Quirk for Serious {}
impl Disadvantage for Serious {}
impl ControlRated for Serious {}
impl ADQ for Serious {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Serious"
    }
}
