use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Dreamer;

impl Quirk for Dreamer {}
impl Disadvantage for Dreamer {}

impl ADQ for Dreamer {
    fn name(&self) -> &str {
        "Dreamer"
    }

    fn cost(&self) -> f64 {
        -1.0
    }
}

impl ControlRated for Dreamer {}
