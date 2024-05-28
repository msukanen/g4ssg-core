use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Responsive;

impl Quirk for Responsive {}
impl Disadvantage for Responsive {}
impl ControlRated for Responsive {}
impl ADQ for Responsive {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Responsive"
    }
}
