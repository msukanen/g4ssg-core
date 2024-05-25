use dice::DiceExt;

use crate::life::{habitat::{ArcticOrDesert, Habitat}, locomotion::Locomotion};

use super::DamageResistance;

pub enum Exoskeleton {
    Light,
    Tough,
    Heavy,
    ArmorShell
}

impl Exoskeleton {
    pub fn random(habitat: &Habitat, locomotion: &Locomotion) -> Exoskeleton {
        let modifier
         = if habitat.is_arctic() {1} else {0}
         + if locomotion.is_immobile() {1}
           else if locomotion.is_flyer() {-2}
           else {0};
        match 2.d6() + modifier {
            ..=2 => Exoskeleton::Light,
            3|4 => Exoskeleton::Tough,
            5 => Exoskeleton::Heavy,
            6.. => Exoskeleton::ArmorShell
        }
    }
}

impl DamageResistance for Exoskeleton {
    fn dr(&self) -> i32 {
        match self {
            Exoskeleton::Light => 0,
            Exoskeleton::Tough => 1,
            Exoskeleton::Heavy => 3,
            Exoskeleton::ArmorShell => 5,
        }
    }
}
