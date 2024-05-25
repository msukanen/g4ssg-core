use dice::DiceExt;

use super::{habitat::Habitat, locomotion::Locomotion, skeleton::Skeleton, trophiclevel::TrophicLevel};

pub enum SkinCovering {
    Exoskeleton,
    Feathers,
    Fur,
    Naked,
    Scales,
}

impl SkinCovering {
    pub fn random(skeleton: &Skeleton) -> SkinCovering {
        if skeleton.is_exoskeleton() {
            SkinCovering::Exoskeleton
        } else {
            match 1.d6() {
                ..=2 => SkinCovering::Naked,
                3 => SkinCovering::Scales,
                4 => SkinCovering::Fur,
                5 => SkinCovering::Feathers,
                6.. => Self::Exoskeleton
            }
        }
    }
}

pub enum SkinToughness {
    Soft,
    Normal,
    Hide,
    Thick,
    ArmorShell,
    Blubber,
}

impl SkinToughness {
    pub fn random(habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> SkinToughness {
        let modifier = if habitat.is_arctic() {1} else if habitat.is_desert() {-1} else {0}
            + match habitat {
                Habitat::Water(_) => 1,
                _ => 0
            } + if locomotion.is_flyer() {-5} else {0}
            + 
    }
}
