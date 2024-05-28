pub trait ADQ {
    fn name(&self) -> &str;
    fn cost(&self) -> f64;
}

pub trait ControlRated {
    fn control(&self) -> i32 {
        15 // NOTE: actual disadvantages must define their own; default value is for Quirk(s).
    }
}

pub trait StatModifier {
    fn stat_modifier(&self) -> i32;
}
