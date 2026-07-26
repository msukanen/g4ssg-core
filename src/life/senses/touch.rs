//! Touch(é).

use dicebag::{DiceExt, lo};
use serde::{Deserialize, Serialize};
use crate::life::{bodyplan::{BodyPlan, Skeleton}, habitat::Habitat, locomotion::Locomotion, senses::special::SpecialSense, size::Size, trophics::TrophicLevel};

use super::{PrimarySense, vision::Vision};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Touch {
    Poor { penalty: u8 },
    HumanLevel,
    Acute { grade: u8 },
} impl Touch {
    pub(crate) fn random(
        primary: PrimarySense,
        size: Size,
        habitat: Habitat,
        trophics: TrophicLevel,
        locomotion: Locomotion,
        bodyplan: &BodyPlan,
        vision: Option<Vision>,
    ) -> Option<(Self, SpecialSense)> {
        let mut modf = primary.modf(PrimarySense::TTS) + if matches!(size, Size::Small { .. }) {1} else {0};
        modf += match bodyplan.skeleton {
            Some(Skeleton::External) => -2,
            _ => 0
        };
        if matches!(habitat, Habitat::Water(_)) {
            modf += 2;
        }
        if locomotion.contains(Locomotion::DIGGING) {
            modf += 2;
        }
        else if locomotion.intersects(Locomotion::WINGED_FLIGHT|Locomotion::BUOYANT_FLIGHT) {
            modf -= 2;
        }
        modf += match vision {
            None |
            Some(Vision::SenseLightAndDark) => 2,
            _ => 0
        };
        if trophics.contains(TrophicLevel::TRAPPING) {
            modf += 1;
        }

        let mut s = SpecialSense::empty();
        (match 2.d6() + modf {
            ..=2 => return None,
            3|4  => Self::Poor { penalty: 2 },
            5|6  => Self::Poor { penalty: 1 },
            7|8  => Self::HumanLevel,
            9|10 => Self::Acute { grade: 4 },
            _    => {
                s = if lo!() { SpecialSense::SENSITIVE_TOUCH } else { SpecialSense::VIBRATION_SENSE };
                Self::Acute { grade: 4 }
            }
        }, s).into()
    }
}
