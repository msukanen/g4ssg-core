use crate::life::adq::{ControlRated, ADQ};

use super::Disadvantage;

pub struct Cowardice;

impl Disadvantage for Cowardice {}
impl ControlRated for Cowardice {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for Cowardice {
    fn cost(&self) -> f64 {
        -10.0
    }

    fn name(&self) -> &str {
        "Cowardice"
    }
}
