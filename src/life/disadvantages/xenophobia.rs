use crate::life::adq::{cost_from_control, ADQ};

use super::{ControlRated, Disadvantage};

pub struct Xenophobia;

impl Disadvantage for Xenophobia {}
impl ADQ for Xenophobia {
    fn cost(&self) -> f64 {
        -cost_from_control(self)
    }

    fn name(&self) -> &str {
        "Xenophobia"
    }
}
impl ControlRated for Xenophobia {
    fn control(&self) -> i32 {
        9
    }
}
