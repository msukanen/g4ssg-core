pub mod naked;
pub mod scales;
pub mod fur;
pub mod feathers;
pub mod exoskeleton;

use dicebag::DiceExt;

use self::{exoskeleton::Exoskeleton, feathers::Feathers, fur::Fur, naked::Naked, scales::Scales};

use super::{habitat::Habitat, locomotion::Locomotion, skeleton::Skeleton, trophiclevel::TrophicLevel};

pub trait DamageResistance {
    fn dr(&self) -> i32;
}

pub enum Skin {
    Exoskeleton(Exoskeleton),
    Feathers(Feathers),
    Fur(Fur),
    Naked(Naked),
    Scales(Scales),
}

impl Skin {
    pub fn random(skeleton: &Skeleton, habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> Skin {
        if skeleton.is_exoskeleton() {
            Skin::Exoskeleton(Exoskeleton::random(habitat, locomotion))
        } else {
            match 1.d6() {
                ..=2 => Skin::Naked(Naked::random(habitat, trophiclevel, locomotion)),
                3 => Skin::Scales(Scales::random(habitat, trophiclevel, locomotion)),
                4 => Skin::Fur(Fur::random(habitat, trophiclevel, locomotion)),
                5 => Skin::Feathers(Feathers::random(habitat, locomotion)),
                6.. => Skin::Exoskeleton(Exoskeleton::random(habitat, locomotion))
            }
        }
    }
}

impl DamageResistance for Skin {
    fn dr(&self) -> i32 {
        match self {
            Skin::Exoskeleton(a) => a.dr(),
            Skin::Feathers(a) => a.dr(),
            Skin::Fur(a) => a.dr(),
            Skin::Naked(a) => a.dr(),
            Skin::Scales(a) => a.dr()
        }
    }
}
