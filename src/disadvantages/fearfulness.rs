use crate::adq::{cost_from_level, ControlRated, ADQ};

use super::Disadvantage;

pub struct Fearfulness {
    level: i32,
    name: String,
}

impl Fearfulness {
    pub fn new(mut level: i32) -> Fearfulness {
        if level < 1 {
            level = 1
        } else if level > 2 {
            level = 2
        }

        Fearfulness {
            name: format!("Fearfulness{}", if level == 1 {" I"} else {" II"}),
            level,
        }
    }
}

impl Disadvantage for Fearfulness {}
impl ControlRated for Fearfulness {}
impl ADQ for Fearfulness {
    fn cost(&self) -> f64 {
        -cost_from_level(self.level)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
