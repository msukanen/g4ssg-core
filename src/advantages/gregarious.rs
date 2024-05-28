use crate::adq::{ControlRated, ADQ};

use super::Advantage;

pub struct Gregarious;

impl Advantage for Gregarious {}
impl ControlRated for Gregarious {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for Gregarious {
    fn cost(&self) -> f64 {
        10.0
    }

    fn name(&self) -> &str {
        "Gregarious"
    }
}
