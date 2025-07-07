use dicebag::DiceExt;

pub enum OrbitalSeparation {
    VeryClose,
    Close,
    Moderate,
    Wide,
    Distant,
}

impl OrbitalSeparation {
    pub fn radius_multipler(&self) -> f64 {
        match self {
            Self::VeryClose => 0.05,
            Self::Close => 0.5,
            Self::Moderate => 2.0,
            Self::Wide => 10.0,
            Self::Distant => 50.0,
        }
    }

    pub fn new(trinary_or_further: bool) -> OrbitalSeparation {
        match 3.d6() + if trinary_or_further {6} else {0} {
            ..=6 => Self::VeryClose,
            7..=9 => Self::Close,
            10|11 => Self::Moderate,
            12..=14 => Self::Wide,
            _ => Self::Distant
        }
    }
}
