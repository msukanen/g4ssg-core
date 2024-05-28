use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Nosy;

impl Quirk for Nosy {}
impl Disadvantage for Nosy {}
impl ADQ for Nosy {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Nosy"
    }
}
impl ControlRated for Nosy {}
