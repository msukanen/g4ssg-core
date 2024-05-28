use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Imaginative;

impl Quirk for Imaginative {}
impl Disadvantage for Imaginative {}
impl ControlRated for Imaginative {}
impl ADQ for Imaginative {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Imaginative"
    }
}
