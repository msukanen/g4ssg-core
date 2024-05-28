use crate::adq::{ControlRated, ADQ};

pub mod racialintolerance;
pub mod xenophobia;
pub mod xenophilia;
pub mod bloodlust;
pub mod orh;
pub mod shortattspan;
pub mod curious;
pub mod incurious;
pub mod selfish;
pub mod oblivious;
pub mod callous;
pub mod loner;
pub mod hidebound;

pub trait Disadvantage: ADQ + ControlRated {}

pub trait DisadvantageContainer: Sized {
    fn disadvantages(&self) -> &Vec<Box<dyn Disadvantage>>;
}

pub(crate) fn cost_from_control(cr: &impl ControlRated) -> f64 {
    match cr.control() {
        15 => 1.0,
        12 => 5.0,
        9 => 10.0,
        _ => 15.0
    }
}

pub(crate) fn cost_from_level(level: i32) -> f64 {
    match level {
        ..=0 => 0.0,
        1 => 5.0,
        2 => 10.0,
        _ => 15.0
    }
}

pub trait ControlAsModifier: ControlRated {
    fn modifier_from_control(&self) -> f64 {
        match self.control() {
            15 => 0.5,
            12 => 1.0,
            9 => 2.0,
            _ => 3.0
        }
    }
}

pub(crate) fn normalize_control(control: i32) -> i32 {
    if      control > 12 {15}
    else if control >  9 {12}
    else if control >  6 { 9}
    else                 { 6}
}
