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
        
    }
}
