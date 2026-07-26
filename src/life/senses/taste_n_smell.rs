//! Taste and smell; oftentimes inseparable for part.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{habitat::Habitat, locomotion::Locomotion, reproduction::Reproduction, senses::special::SpecialSense, trophics::TrophicLevel};

use super::PrimarySense;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum TasteAndSmell {
    NoSmell,
    Normal,
    AcuteTaste { grade: u8 },
    AcuteTnS { grade: u8 },
} impl TasteAndSmell {
    pub(crate) fn random(
        primary: PrimarySense,
        trophics: TrophicLevel,
        habitat: Habitat,
        locomotion: Locomotion,
        reproduction: &Reproduction
    ) -> Option<(Self, SpecialSense)> {
        use TrophicLevel as TL;
        let mut modf = primary.modf(PrimarySense::TTS) + if locomotion.is_empty() {-4} else {0};
        if reproduction.arrangement.is_sexual() {
            modf += 2;
        }
        if trophics.intersects(TL::CHASING | TL::GATHERING) {
            modf += 2;
        }
        else if trophics.intersects(TL::FILTER_FEEDER | TL::AUTOTROPH | TL::TRAPPING) {
            modf -= 2;
        }
        let mut s = SpecialSense::empty();
        (match 2.d6() + modf {
            ..=3 => return None,
            4|5  => Self::NoSmell,
            ..=8 => Self::Normal,
            9|10 => if matches!(habitat, Habitat::Water(_))
                         { Self::AcuteTaste { grade: 4 }}
                    else { Self::AcuteTnS { grade: 4 }},
            _    => if matches!(habitat, Habitat::Water(_))
                         { s = SpecialSense::DISCRIMINATORY_TASTE; Self::AcuteTaste { grade: 4 }}
                    else { s = SpecialSense::DISCRIMINATORY_SMELL; Self::AcuteTnS { grade: 4 }}
        }, s).into()
    }
}
