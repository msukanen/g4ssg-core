use crate::life::adq::{ControlRated, ADQ};

use super::Advantage;

pub struct HighPainThreshold;

impl Advantage for HighPainThreshold {}
impl ControlRated for HighPainThreshold {
    fn control(&self) -> i32 {
        15
    }
}
impl ADQ for HighPainThreshold {
    fn cost(&self) -> f64 {
        5.0
    }

    fn name(&self) -> &str {
        "High Pain Threshold"
    }
}
