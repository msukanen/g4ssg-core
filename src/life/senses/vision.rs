//! Visual ability, if any.

use dicebag::{DiceExt, lo};
use serde::{Deserialize, Serialize};
use crate::life::{habitat::{Habitat, WaterHabitat}, locomotion::Locomotion, trophics::TrophicLevel};

use super::PrimarySense;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Vision {
    BadSight { colorblind: bool },
    ColorBlind,
    Normal,
    SenseLightAndDark,
    Telescopic,
} impl Vision {
    pub(crate) fn random(
        primary: PrimarySense,
        habitat: Habitat,
        trophics: TrophicLevel,
        locomotion: Locomotion
    ) -> Option<Self> {
        let mut modf = primary.modf(PrimarySense::Vision);

        if locomotion.is_empty() || locomotion.contains(Locomotion::DIGGING) {
            modf -= 4;
        }
        if locomotion.intersects(Locomotion::WINGED_FLIGHT | Locomotion::BUOYANT_FLIGHT) {
            modf += 3;
        }
        else if locomotion.contains(Locomotion::CLIMBING) {
            modf += 2;
        }
        if matches!(habitat, Habitat::Water(WaterHabitat::DeepOceanVents)) {
            modf -= 4;
        }
        if trophics.contains(TrophicLevel::FILTER_FEEDER) {
            modf -= 2;
        }
        if trophics.intersects(TrophicLevel::CARNIVORE | TrophicLevel::GATHERING) {
            modf += 2;
        }
        
        let mut r = 2.d6() + modf;
        if matches!(habitat, Habitat::Space(_)) && r <= 9 { r = 3 }
        match r {
            ..=6 => return None,
            7    => Self::SenseLightAndDark,
            8|9  => Self::BadSight { colorblind: true },
            10|11 => if lo!() { Self::ColorBlind } else { Self::BadSight { colorblind: false } },
            ..=14 => Self::Normal,
            _     => Self::Telescopic
        }.into()
    }
}
