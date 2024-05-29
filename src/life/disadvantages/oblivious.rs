use crate::life::adq::{ControlRated, ADQ};

use super::Disadvantage;

pub struct Oblivious;

impl Disadvantage for Oblivious {}
impl ControlRated for Oblivious {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for Oblivious {
    fn cost(&self) -> f64 {
        -10.0
    }

    fn name(&self) -> &str {
        "Oblivious"
    }
}
