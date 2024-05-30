use dice::DiceExt;

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
pub enum OrbitalSeparation {
    VeryClose,
    Close,
    Moderate,
    Wide,
    Distant
}

impl OrbitalSeparation {
    pub fn radius_multiplier(&self) -> f64 {
        match self {
            Self::VeryClose => 0.05,
            Self::Close => 0.5,
            Self::Moderate => 2.0,
            Self::Wide => 10.0,
            Self::Distant => 50.0
        }
    }

    pub fn random(second_companion_in_trinary: bool, modifier: i32) -> OrbitalSeparation {
        match 3.d6() + if second_companion_in_trinary {6} else {0} + modifier {
            ..=6 => Self::VeryClose,
            7..=9 => Self::Close,
            10|11 => Self::Moderate,
            12..=14 => Self::Wide,
            15.. => Self::Distant
        }
    }

    pub fn eccentricity_modifier(&self) -> i32 {
        match self {
            Self::VeryClose => -6,
            Self::Close => -4,
            Self::Moderate => -2,
            _ => 0
        }
    }
}
