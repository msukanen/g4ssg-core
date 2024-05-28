use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Humble;

impl Quirk for Humble {}
impl Disadvantage for Humble {}
impl ADQ for Humble {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Humble"
    }
}
impl ControlRated for Humble {}
