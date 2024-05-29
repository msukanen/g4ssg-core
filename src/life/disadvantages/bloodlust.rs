use crate::life::adq::ADQ;

use super::{ControlRated, Disadvantage};

pub struct Bloodlust;

impl Disadvantage for Bloodlust {}

impl ADQ for Bloodlust {
    fn cost(&self) -> f64 {
        -10.0
    }

    fn name(&self) -> &str {
        "Bloodlust"
    }
}

impl ControlRated for Bloodlust {
    fn control(&self) -> i32 {
        12
    }
}
