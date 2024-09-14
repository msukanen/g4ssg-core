use dicebag::DiceExt;

use crate::life::{habitat::{ArcticOrDesert, Habitat}, locomotion::{Locomotion, LocomotionMode}, trophiclevel::TrophicLevel};

use super::DamageResistance;

pub enum Scales {
    Skinlike,
    Normal,
    Heavy,
    ArmorShell
}

impl Scales {
    pub fn random(habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> Scales {
        let modifier = if habitat.is_desert() {1} else {0}
            + if trophiclevel.is_herbivore(None) {1} else {0}
            + if locomotion.is_flyer() {-2}
              else if locomotion.is(LocomotionMode::Digging) {-1}
              else {0};
        match 2.d6() + modifier {
            ..=3 => Scales::Skinlike,
            4..=8 => Scales::Normal,
            9|10 => Scales::Heavy,
            11.. => Scales::ArmorShell
        }
    }
}

impl DamageResistance for Scales {
    fn dr(&self) -> i32 {
        match self {
            Scales::Skinlike => 0,
            Scales::Normal => 1,
            Scales::Heavy => 3,
            Scales::ArmorShell => 5,
        }
    }
}
