use crate::life::adq::{ControlRated, ADQ};

use super::Disadvantage;

pub struct Trickster;

impl Disadvantage for Trickster {}
impl ControlRated for Trickster {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for Trickster {
    fn cost(&self) -> f64 {
        -15.0
    }

    fn name(&self) -> &str {
        "Trickster (12)"
    }
}
