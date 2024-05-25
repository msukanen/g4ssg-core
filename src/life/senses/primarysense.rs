use dice::DiceExt;

use crate::life::{habitat::Habitat, trophiclevel::TrophicLevel};

pub enum PrimarySense {
    Hearing,
    Vision,
    TouchAndTaste
}

impl PrimarySense {
    pub fn random(habitat: &Habitat, trophiclevel: &TrophicLevel) -> PrimarySense {
        let modifier = match habitat {
            Habitat::Water(_) => -2,
            _ => 0
        } + if trophiclevel.is_autotroph() {2} else {0};
        match 3.d6() + modifier {
            ..=7 => Self::Hearing,
            8..=12 => Self::Vision,
            13.. => Self::TouchAndTaste
        }
    }
}
