use crate::adq::{ControlRated, ADQ};

use super::Advantage;

pub struct Chummy;

impl Advantage for Chummy {}
impl ControlRated for Chummy {
    fn control(&self) -> i32 {
        15
    }
}
impl ADQ for Chummy {
    fn cost(&self) -> f64 {
        5.0
    }

    fn name(&self) -> &str {
        "Chummy"
    }
}
