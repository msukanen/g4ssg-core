use crate::life::adq::{cost_from_control, normalize_control, ControlRated, ADQ};

use super::Disadvantage;

pub struct CompulsivePlayfulness {
    control: i32,
    name: String,
}

impl CompulsivePlayfulness {
    pub fn new(mut control: i32) -> CompulsivePlayfulness {
        control = normalize_control(control);
        CompulsivePlayfulness {
            name: format!("Compulsive Playfulness {control}"),
            control
        }
    }
}

impl Disadvantage for CompulsivePlayfulness {}
impl ControlRated for CompulsivePlayfulness {
    fn control(&self) -> i32 {
        self.control
    }
}
impl ADQ for CompulsivePlayfulness {
    fn cost(&self) -> f64 {
        -cost_from_control(self)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
