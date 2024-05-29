use crate::life::adq::{ControlRated, ADQ};

use super::Disadvantage;

pub struct Paranoia;

impl Disadvantage for Paranoia {}
impl ControlRated for Paranoia {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for Paranoia {
    fn cost(&self) -> f64 {
        -15.0
    }

    fn name(&self) -> &str {
        "Paranoia"
    }
}
