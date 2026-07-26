//! Senses, primary and otherwise.

use astrometrics::MetricsInternalType;
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{bodyplan::BodyPlan, chemistry::ChemicalBasis, habitat::Habitat, locomotion::Locomotion, reproduction::Reproduction, size::Size, trophics::TrophicLevel};

pub mod hearing; use hearing::Hearing;
pub mod special; use special::SpecialSense;
pub mod taste_n_smell; use taste_n_smell::TasteAndSmell;
pub mod touch; use touch::Touch;
pub mod vision; use vision::Vision;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Senses {
    pub hearing: Option<Hearing>,
    pub taste_smell: Option<TasteAndSmell>,
    pub touch: Option<Touch>,
    pub special: SpecialSense,
    pub vision: Option<Vision>,
} impl Senses {
    pub fn random(
        g: Option<MetricsInternalType>,
        cb: ChemicalBasis,
        size: Size,
        habitat: Habitat,
        trophics: TrophicLevel,
        locomotion: Locomotion,
        bodyplan: &BodyPlan,
        reproduction: &Reproduction,
    ) -> Self {
        use PrimarySense as PS;
        let primary = PS::random(habitat, trophics);
        let vision = Vision::random(primary, habitat, trophics, locomotion);
        let mut special = SpecialSense::empty();
        let hearing = if let Some((hearing, s)) = Hearing::random(primary, size, habitat, locomotion, vision) {
            special |= s;
            hearing.into()
        } else {None};
        let touch = if let Some((touch, s)) = Touch::random(primary, size, habitat, trophics, locomotion, bodyplan, vision) {
            special |= s;
            touch.into()
        } else {None};
        let taste_smell = if let Some((tts, s)) = TasteAndSmell::random(primary, trophics, habitat, locomotion, reproduction) {
            special |= s;
            tts.into()
        } else {None};
        special |= SpecialSense::random(false, g, cb, size, habitat, trophics, locomotion, bodyplan, vision, hearing);
        Self {
            vision,
            hearing,
            taste_smell,
            touch,
            special,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum PrimarySense {
    Hearing,
    /// Touch, Taste, and/or Smell
    TTS,
    Vision,
} impl PrimarySense {
    fn random(habitat: Habitat, trophics: TrophicLevel) -> Self {
        let mut modf = if trophics.contains(TrophicLevel::AUTOTROPH) {2} else {0};
        if matches!(habitat, Habitat::Water(_)) {
            modf -= 2;
        }
        match 3.d6() + modf {
            ..=7  => Self::Hearing,
            ..=12 => Self::Vision,
            _     => Self::TTS
        }
    }

    #[inline(always)]
    const fn modf(&self, maybe: PrimarySense) -> i32 {
        match (self, maybe) {
            (Self::Hearing, Self::Hearing) |
            (Self::TTS, Self::TTS)         |
            (Self::Vision, Self::Vision)   => 4,
            _ => 0
        }
    }
}
