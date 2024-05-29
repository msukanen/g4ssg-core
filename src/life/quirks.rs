use crate::life::disadvantages::Disadvantage;

use super::adq::{ControlRated, ADQ};

pub enum Quirks {
    Attentive,
    BroadMinded,
    Careful,
    Chauvinistic,
    Congenial,
    Distractible,
    Dreamer,
    Dull,
    Humble,
    Imaginative,
    Nosy,
    Playful,
    Proud,
    Responsive,
    Serious,
    Staid,
    Uncongenial,
    Undiscriminating,
    Versatile,
}

pub trait Quirk: Disadvantage {}

impl Quirk for Quirks {}
impl Disadvantage for Quirks {}
impl ControlRated for Quirks {
    fn control(&self) -> i32 {
        15
    }
}
impl ADQ for Quirks {
    fn cost(&self) -> f64 {
        -1.0
    }

    fn name(&self) -> &str {
        match self {
            Self::Attentive => "Attentive",
            Self::BroadMinded => "Broad-minded",
            Self::Careful => "Careful",
            Self::Chauvinistic => "Chauvinistic",
            Self::Congenial => "Congenial",
            Self::Distractible => "Distractible",
            Self::Dreamer => "Dreamer",
            Self::Dull => "Dull",
            Self::Humble => "Humble",
            Self::Imaginative => "Imaginative",
            Self::Nosy => "Nosy",
            Self::Playful => "Playful",
            Self::Proud => "Proud",
            Self::Responsive => "Responsive",
            Self::Serious => "Serious",
            Self::Staid => "Staid",
            Self::Uncongenial => "Uncongenial",
            Self::Undiscriminating => "Undiscriminating",
            Self::Versatile => "Versatile",
        }
    }
}
