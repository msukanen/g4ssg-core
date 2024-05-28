use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Distractible;

impl Quirk for Distractible {}
impl Disadvantage for Distractible {}
impl ADQ for Distractible {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Distractible"
    }
}
impl ControlRated for Distractible {}
