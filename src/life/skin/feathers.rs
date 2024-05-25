use dice::DiceExt;

use crate::life::{habitat::{ArcticOrDesert, Habitat}, locomotion::Locomotion};

use super::DamageResistance;

pub enum Feathers {
    BareSkin,
    Normal,
    Thick,
    OverHide,
    Spines
}

impl Feathers {
    pub fn random(habitat: &Habitat, locomotion: &Locomotion) -> Feathers {
        let modifier
         = if habitat.is_arctic() {1}
           else if habitat.is_desert() {-1}
           else {0}
         + if locomotion.is_flyer() {1} else {0};
        match 1.d6() + modifier {
            ..=5 => Feathers::BareSkin,
            6..=8 => Feathers::Normal,
            9|10 => Feathers::Thick,
            11 => Feathers::OverHide,
            12.. => Feathers::Spines
        }
    }
}

impl DamageResistance for Feathers {
    fn dr(&self) -> i32 {
        match self {
            Feathers::BareSkin |
            Feathers::Normal   |
            Feathers::Thick    |
            Feathers::Spines   => 0,
            Feathers::OverHide => 1,
        }
    }
}
