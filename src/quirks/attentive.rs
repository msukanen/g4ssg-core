use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Attentive;

impl Quirk for Attentive {}
impl Disadvantage for Attentive {}
impl ControlRated for Attentive {}
impl ADQ for Attentive {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Attentive"
    }
}
