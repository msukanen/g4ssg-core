use crate::adq::{ControlRated, ADQ};

use super::Disadvantage;

pub struct NoSenseOfHumor;

impl Disadvantage for NoSenseOfHumor {}
impl ControlRated for NoSenseOfHumor {}
impl ADQ for NoSenseOfHumor {
    fn cost(&self) -> f64 {
        -15.0
    }

    fn name(&self) -> &str {
        "No Sense of Humor"
    }
}
