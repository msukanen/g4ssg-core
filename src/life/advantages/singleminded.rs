use crate::life::adq::{ControlRated, ADQ};

use super::Advantage;

pub struct SingleMinded;

impl Advantage for SingleMinded {}
impl ControlRated for SingleMinded {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for SingleMinded {
    fn cost(&self) -> f64 {
        10.0
    }

    fn name(&self) -> &str {
        "Single-minded"
    }
}
