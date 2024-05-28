use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Dull;

impl Quirk for Dull {}
impl Disadvantage for Dull {}
impl ControlRated for Dull {}
impl ADQ for Dull {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Dull"
    }
}
