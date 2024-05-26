use self::{hearing::Hearing, primarysense::PrimarySense, tastesmell::TasteSmell, touch::Touch, vision::Vision};

use super::{habitat::Habitat, locomotion::Locomotion, sex::Reproduction, size::SizeCategory, skeleton::Skeleton, trophiclevel::TrophicLevel};

pub mod primarysense;
pub mod vision;
pub mod hearing;
pub mod touch;
pub mod tastesmell;

pub struct Senses {
    primary_sense: PrimarySense,
    vision: Vision,
    hearing: Hearing,
    touch: Touch,
    taste_smell: TasteSmell,
}

impl Senses {
    pub fn random(size_category: &SizeCategory, habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion, skeleton: &Skeleton, reproduction: &Reproduction) -> Senses {
        let primary_sense = PrimarySense::random(habitat, trophiclevel);
        let vision = Vision::random(primary_sense == PrimarySense::Vision, habitat, trophiclevel, locomotion);
        let hearing = Hearing::random(primary_sense == PrimarySense::Hearing, &vision, size_category, habitat, locomotion);
        let touch = Touch::random(primary_sense == PrimarySense::TouchAndTaste, &vision, size_category, habitat, trophiclevel, locomotion, skeleton);
        let taste_smell = TasteSmell::random(primary_sense == PrimarySense::TouchAndTaste, habitat, trophiclevel, locomotion, reproduction);
        Senses { primary_sense, vision, hearing, touch, taste_smell }
    }
}
