use crate::adq::{cost_from_level, ControlRated, ADQ};

use super::Advantage;

pub struct Fearlessness {
    level: i32,
    name: String,
}

impl Fearlessness {
    pub fn new(mut level: i32) -> Fearlessness {
        level = if level < 1 {1} else if level > 3 {3} else {level};
        Fearlessness {
            name: format!("Fearlessness{}", match level {
                ..=1 => " I",
                2 => " II",
                _ => " III",
            }),
            level
        }
    }
}

impl Advantage for Fearlessness {}
impl ControlRated for Fearlessness {}
impl ADQ for Fearlessness {
    fn name(&self) -> &str {
        &self.name
    }

    fn cost(&self) -> f64 {
        cost_from_level(self.level)
    }
}

pub struct Unfazeable;

impl Advantage for Unfazeable {}
impl ControlRated for Unfazeable {}
impl ADQ for Unfazeable {
    fn cost(&self) -> f64 {
        15.0
    }

    fn name(&self) -> &str {
        "Unfazeable"
    }
}
