use dicebag::DiceExt;

use crate::life::{habitat::{ArcticOrDesert, Habitat}, locomotion::Locomotion, trophiclevel::TrophicLevel};

use super::DamageResistance;

pub enum Naked {
    Soft,
    Normal,
    Hide,
    Thick,
    ArmorShell,
    Blubber,
}

impl Naked {
    pub fn random(habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> Naked {
        let modifier = if habitat.is_arctic() {1} else if habitat.is_desert() {-1} else {0}
            + match habitat {
                Habitat::Water(_) => 1,
                _ => 0
            } + if locomotion.is_flyer() {-5} else {0}
            + if trophiclevel.is_herbivore(None) {1} else {0};
        match 2.d6() + modifier {
            ..=4 => Naked::Soft,
            5 => Naked::Normal,
            6|7 => Naked::Hide,
            8 => Naked::Thick,
            9 => Naked::ArmorShell,
            10.. => Naked::Blubber
        }
    }
}


impl DamageResistance for Naked {
    fn dr(&self) -> i32 {
        match self {
            Naked::Soft    |
            Naked::Normal  => 0,
            Naked::Hide    => 1,
            Naked::Thick   |
            Naked::Blubber => 4,
            Naked::ArmorShell => 5,
        }
    }
}
