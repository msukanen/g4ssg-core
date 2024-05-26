use self::{hearing::Hearing, primarysense::PrimarySense, touch::Touch, vision::Vision};

use super::{habitat::Habitat, locomotion::Locomotion, size::SizeCategory, skeleton::Skeleton, trophiclevel::TrophicLevel};

pub mod primarysense;
pub mod vision;
pub mod hearing;
pub mod touch;

pub struct Senses {
    primary_sense: PrimarySense,
    vision: Vision,
    hearing: Hearing,
    touch: Touch,
    taste_smell: TasteSmell,
}

impl Senses {
    pub fn random(size_category: &SizeCategory, habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion, skeleton: &Skeleton ) -> Senses {
        let primary_sense = PrimarySense::random(habitat, trophiclevel);
        let vision = Vision::random(primary_sense == PrimarySense::Vision, habitat, trophiclevel, locomotion);
        let touch = Touch::random(primary_sense == PrimarySense::TouchAndTaste, &vision, size_category, habitat, trophiclevel, locomotion, skeleton);
    }
}
