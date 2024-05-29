use crate::life::adq::{ControlRated, ADQ, CONTROL_ALWAYS_ON};

use super::Disadvantage;

pub struct NoSenseOfHumor;

impl Disadvantage for NoSenseOfHumor {}
impl ControlRated for NoSenseOfHumor {
    fn control(&self) -> i32 {
        CONTROL_ALWAYS_ON
    }
}
impl ADQ for NoSenseOfHumor {
    fn cost(&self) -> f64 {
        -15.0
    }

    fn name(&self) -> &str {
        "No Sense of Humor"
    }
}
