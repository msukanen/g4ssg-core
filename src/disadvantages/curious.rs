use crate::adq::{cost_from_level, normalize_control, ControlAsModifier, ADQ};

use super::{ControlRated, Disadvantage};

pub struct Curious {
    level: i32,
    name: String,
    control: i32,
}

impl Curious {
    pub fn new(mut control: i32, level: i32) -> Curious {
        control = normalize_control(control);
        Curious {
            name: format!("Curious {}({control}", match level {
                ..=1 => "",
                2 => "II ",
                _ => "III "
            }),
            level,
            control
        }
    }
}

impl Disadvantage for Curious {}
impl ADQ for Curious {
    fn cost(&self) -> f64 {
        let a = cost_from_level(self.level);
        let b = self.modifier_from_control();
        -(a * b)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
impl ControlRated for Curious {
    fn control(&self) -> i32 {
        self.control
    }
}

impl ControlAsModifier for Curious {}
