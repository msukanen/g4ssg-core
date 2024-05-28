use crate::adq::ADQ;

use super::{cost_from_control, normalize_control, ControlRated, Disadvantage};

pub struct ShortAttentionSpan {
    name: String,
    control: i32,
}

impl ControlRated for ShortAttentionSpan {
    fn control(&self) -> i32 {
        self.control
    }
}

impl ShortAttentionSpan {
    pub fn new(mut control: i32) -> ShortAttentionSpan {
        control = normalize_control(control);
        ShortAttentionSpan {
            name: format!("Short Attention Span ({})", &control),
            control
        }
    }
}

impl Disadvantage for ShortAttentionSpan {}
impl ADQ for ShortAttentionSpan {
    fn name(&self) -> &str {
        &self.name
    }

    fn cost(&self) -> f64 {
        -cost_from_control(self)
    }
}
