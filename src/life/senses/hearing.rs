use dice::{DiceExt, HiLo};

use crate::life::{habitat::Habitat, locomotion::Locomotion, size::SizeCategory};

use super::vision::Vision;

pub enum ExtraHearingRange {
    Infrasound,
    Ultrasonic,
    Sonar,
}

pub enum Hearing {
    Deaf,
    HardOfHearing,
    Normal(Option<ExtraHearingRange>),
    Acute(Option<ExtraHearingRange>, Option<ExtraHearingRange>)
}

impl Hearing {
    pub fn random(primary_sense: bool, vision: &Vision, size_category: &SizeCategory, habitat: &Habitat, locomotion: &Locomotion) -> Hearing {
        if habitat.is_space() {
            return Self::Deaf
        }

        let modifier = match vision {
            Vision::Blindness(_) => 2,
            Vision::BadSight(_) => 1,
            _ => 0
          } + if locomotion.is_immobile() {4} else {0}
            + if habitat.is_aquatic() {1} else {0}
            + if primary_sense {4} else {0};
        match 3.d6() + modifier {
            ..=6 => Self::Deaf,
            7|8 => Self::HardOfHearing,
            9|10 => Self::Normal(None),
            11 => Self::Normal(Some(if size_category == &SizeCategory::Large {ExtraHearingRange::Infrasound} else {ExtraHearingRange::Ultrasonic})),
            12 => Self::Acute(None, None),
            13 => Self::Acute(Some(if 1.d2().lo() {ExtraHearingRange::Infrasound} else {ExtraHearingRange::Ultrasonic}), None),
            14.. => Self::Acute(Some(ExtraHearingRange::Ultrasonic), Some(ExtraHearingRange::Sonar))
        }
    }

    pub fn has_sonar(&self) -> bool {
        match self {
            Self::Acute(_, Some(ExtraHearingRange::Sonar)) => true,
            _ => false
        }
    }
}
