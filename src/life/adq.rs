pub trait ADQ {
    fn name(&self) -> &str;
    fn cost(&self) -> f64;
}

pub trait ControlRated {
    fn control(&self) -> i32;
}

pub const CONTROL_ALWAYS_ON: i32 = 666;

pub trait StatModifier {
    fn stat_modifier(&self) -> i32;
}

pub(crate) fn cost_from_control(cr: &impl ControlRated) -> f64 {
    match cr.control() {
        12 => 5.0,
        9 => 10.0,
        6 => 15.0,
        _ => 1.0
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
            12 => 1.0,
            9 => 2.0,
            6 => 3.0,
            _ => 0.5,
        }
    }
}

pub(crate) fn normalize_control(control: i32) -> i32 {
    if      control > 12 {15}
    else if control >  9 {12}
    else if control >  6 { 9}
    else                 { 6}
}
