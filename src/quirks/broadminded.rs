use crate::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

use super::Quirk;

pub struct BroadMinded;

impl Quirk for BroadMinded {}
impl Disadvantage for BroadMinded {}
impl ControlRated for BroadMinded {}
impl ADQ for BroadMinded {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        "Broad-minded"
    }
}
