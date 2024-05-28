use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Congenial;
pub struct Uncongenial;

impl Quirk for Congenial {}
impl Quirk for Uncongenial {}

impl Disadvantage for Congenial {}
impl Disadvantage for Uncongenial {}

impl ControlRated for Congenial {}
impl ControlRated for Uncongenial {}

impl ADQ for Congenial {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Congenial"
    }
}
impl ADQ for Uncongenial {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Uncongenial"
    }
}
