//! Hearing is important for some beings.
use dicebag::{DiceExt, lo};
use serde::{Deserialize, Serialize};
use crate::life::{habitat::Habitat, locomotion::Locomotion, senses::special::SpecialSense, size::Size};
use super::{PrimarySense, vision::Vision};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Hearing {
    HardOfHearing,
    Normal,
    Extended { subsonic: bool, ultrasonic: bool },
    Acute { subsonic: bool, ultrasonic: bool, grade: u8, sonar: bool },
} impl Hearing {
    pub(crate) fn random(
        primary: PrimarySense,
        size: Size,
        habitat: Habitat,
        locomotion: Locomotion,
        vision: Option<Vision>,
    ) -> Option<(Self, SpecialSense)> {
        let mut modf = primary.modf(PrimarySense::Hearing);

        modf += match habitat {
            Habitat::Space(_) => {
                return match (3.d6(), 3.d6()) {
                    (17, ..=10) => return None,
                    (17, _) => (Self::HardOfHearing, SpecialSense::empty()),
                    (18, 16..) => (Self::Normal, SpecialSense::empty()),
                    _ => return None
                }.into()
            },
            Habitat::Water(_) => 1,
            _ => 0
        };

        modf += match vision {
            None |
            Some(Vision::SenseLightAndDark) => 2,
            Some(Vision::BadSight { .. }) => 1,
            _ => 0
        };

        if locomotion.is_empty() {
            modf -= 4;
        }

        match 3.d6() + modf {
            ..=6 => return None,
            7|8  => (Self::HardOfHearing, SpecialSense::empty()),
            9|10 => (Self::Normal, SpecialSense::empty()),
            11   => (Self::Extended { subsonic: matches!(size, Size::Large { .. }), ultrasonic: !matches!(size, Size::Large { .. }) }, SpecialSense::empty()),
            12   => (Self::Acute { subsonic: false, ultrasonic: false, grade: 3, sonar: false }, SpecialSense::empty()),
            13   => {
                    let su = lo!();
                    (Self::Acute { subsonic: su, ultrasonic: !su, grade: if lo!() {3} else {4}, sonar: false }, SpecialSense::empty())
                },
            _    => (Self::Acute { subsonic: false, ultrasonic: true, grade: 4, sonar: true }, SpecialSense::DISCRIMINATORY_HEARING)
        }.into()
    }
}
