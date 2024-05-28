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
