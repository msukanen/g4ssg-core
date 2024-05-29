use crate::life::{adq::ADQ, disadvantages::{ControlRated, Disadvantage}};

pub struct NonstopIdeaFactory;

impl Disadvantage for NonstopIdeaFactory {}
impl ADQ for NonstopIdeaFactory {
    fn cost(&self) -> f64 {
        -5.0
    }

    fn name(&self) -> &str {
        "Odious Racial Habit: Nonstop Idea Factory"
    }
}
impl ControlRated for NonstopIdeaFactory {
    fn control(&self) -> i32 {
        12
    }
}
