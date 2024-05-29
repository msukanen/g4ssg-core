use crate::life::{adq::{ControlRated, ADQ}, disadvantages::Disadvantage};

pub struct WetBlanket;

impl Disadvantage for WetBlanket {}
impl ControlRated for WetBlanket {
    fn control(&self) -> i32 {
        15
    }
}
impl ADQ for WetBlanket {
    fn cost(&self) -> f64 {
        -5.0
    }

    fn name(&self) -> &str {
        "Odious Racial Habit: \"Wet Blanket\""
    }
}
