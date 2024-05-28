use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct Chauvinistic;

impl Quirk for Chauvinistic {}
impl Disadvantage for Chauvinistic {}
impl ADQ for Chauvinistic {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Chauvinistic"
    }
}
impl ControlRated for Chauvinistic {}
