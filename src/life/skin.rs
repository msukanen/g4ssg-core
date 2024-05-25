use dice::DiceExt;

use super::{habitat::{ArcticOrDesert, Habitat}, locomotion::{Locomotion, LocomotionMode}, skeleton::Skeleton, trophiclevel::TrophicLevel};

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
            + if trophiclevel.is_herbivore() {1} else {0};
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

pub enum Scales {
    Skinlike,
    Normal,
    Heavy,
    ArmorShell
}

impl Scales {
    pub fn random(habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> Scales {
        let modifier = if habitat.is_desert() {1} else {0}
            + if trophiclevel.is_herbivore() {1} else {0}
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
         + if trophiclevel.is_herbivore() {1} else {0}
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
