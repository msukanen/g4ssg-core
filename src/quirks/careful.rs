use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Careful;

impl Quirk for Careful {}
impl Disadvantage for Careful {}
impl ControlRated for Careful {}
impl ADQ for Careful {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Careful"
    }
}
