use dice::DiceExt;

use crate::life::{habitat::{ArcticOrDesert, Habitat}, locomotion::Locomotion, trophiclevel::TrophicLevel};

use super::DamageResistance;

pub enum Fur {
    BareSkin,
    Normal,
    Thick(bool),
    Spines,
}

impl Fur {
    pub fn random(habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> Fur {
        let modifier
         = if habitat.is_desert() {-1}
           else if habitat.is_arctic() {1}
           else {0}
         + if trophiclevel.is_herbivore(None) {1} else {0}
         + if locomotion.is_flyer() {-1} else {0};
        match 2.d6() + modifier {
            ..=5 => Fur::BareSkin,
            6|7 => Fur::Normal,
            8|9 => Fur::Thick(false),
            10|11 => Fur::Thick(true),
            12.. => Fur::Spines
        }
    }
}

impl DamageResistance for Fur {
    fn dr(&self) -> i32 {
        match self {
            Fur::BareSkin |
            Fur::Normal   |
            Fur::Spines   |
            Fur::Thick(false) => 0,
            Fur::Thick(true)  => 1
        }
    }
}
