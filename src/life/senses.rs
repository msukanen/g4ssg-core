use self::{hearing::Hearing, primarysense::PrimarySense, vision::Vision};

use super::{habitat::Habitat, locomotion::Locomotion, trophiclevel::TrophicLevel};

pub mod primarysense;
pub mod vision;
pub mod hearing;

pub struct Senses {
    primary_sense: PrimarySense,
    vision: Vision,
    hearing: Hearing,
    touch: Touch,
    taste_smell: TasteSmell,
}

impl Senses {
    pub fn random(habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> Senses {
        let primary_sense = PrimarySense::random(habitat, trophiclevel);
        let vision = Vision::random(primary_sense == PrimarySense::Vision, habitat, trophiclevel, locomotion);
    }
}
