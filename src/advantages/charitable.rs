use crate::adq::{ControlRated, ADQ};

use super::Advantage;

pub struct Charitable;

impl Advantage for Charitable {}
impl ControlRated for Charitable {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for Charitable {
    fn cost(&self) -> f64 {
        10.0
    }

    fn name(&self) -> &str {
        "Charitable"
    }
}
