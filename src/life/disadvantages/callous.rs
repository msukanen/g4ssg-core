use crate::life::adq::{ControlRated, ADQ};

use super::Disadvantage;

pub struct Callous;

impl Disadvantage for Callous {}
impl ControlRated for Callous {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for Callous {
    fn cost(&self) -> f64 {
        -10.0
    }

    fn name(&self) -> &str {
        "Callous"
    }
}
