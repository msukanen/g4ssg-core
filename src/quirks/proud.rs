use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Proud;

impl Quirk for Proud {}
impl Disadvantage for Proud {}
impl ADQ for Proud {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Proud"
    }
}
impl ControlRated for Proud {}
