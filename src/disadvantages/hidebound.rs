use crate::adq::{ControlRated, StatModifier, ADQ};

use super::Disadvantage;

pub struct Hidebound {
    iq_penalty: i32,
    name: String
}

impl Hidebound {
    pub fn new(with_iq_penalty: bool) -> Hidebound {
        Hidebound {
            iq_penalty: if with_iq_penalty {-1} else {0},
            name: format!("Hidebound{}", if with_iq_penalty {" (-1 IQ)"} else {""})
        }
    }
}

impl StatModifier for Hidebound {
    fn stat_modifier(&self) -> i32 {
        self.iq_penalty
    }
}

impl Disadvantage for Hidebound {}
impl ControlRated for Hidebound {}
impl ADQ for Hidebound {
    fn cost(&self) -> f64 {
        (-5 + (10 * self.iq_penalty)) as f64
    }

    fn name(&self) -> &str {
        &self.name
    }
}
