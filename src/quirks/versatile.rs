use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Versatile;

impl Quirk for Versatile {}
impl Disadvantage for Versatile {}

impl ADQ for Versatile {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Versatile"
    }
}

impl ControlRated for Versatile {}
