use crate::life::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Advantage;

pub struct Empathy;
pub struct LowEmpathy;
pub struct Sensitive;

impl Advantage for Empathy {}
impl ControlRated for Empathy {
    fn control(&self) -> i32 {
        15
    }
}
impl ADQ for Empathy {
    fn cost(&self) -> f64 {
        15.0
    }

    fn name(&self) -> &str {
        "Empathy"
    }
}

impl Advantage for Sensitive {}
impl ControlRated for Sensitive {
    fn control(&self) -> i32 {
        12
    }
}
impl ADQ for Sensitive {
    fn cost(&self) -> f64 {
        10.0
    }

    fn name(&self) -> &str {
        "Empathy: Sensitive"
    }
}

impl Disadvantage for LowEmpathy {}
impl ControlRated for LowEmpathy {
    fn control(&self) -> i32 {
        15
    }
}
impl ADQ for LowEmpathy {
    fn cost(&self) -> f64 {
        -10.0
    }

    fn name(&self) -> &str {
        "Low Empathy"
    }
}
