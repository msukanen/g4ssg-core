use dicebag::DiceExt;

use crate::life::{habitat::Habitat, locomotion::Locomotion, sex::Reproduction, trophiclevel::{Carnivore, Herbivore, TrophicLevel, TrophicLevelType}};

pub enum TasteSmell {
    NoSense(bool),
    Normal,
    AcuteTasteAndSmell,
    AcuteTaste,
    AcuteTasteAndDiscriminatorySmell,
    DiscriminatoryTaste
}

impl TasteSmell {
    pub fn random(primary_sense: bool, habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion, reproduction: &Reproduction) -> TasteSmell {
        let modifier = if primary_sense {4} else {0}
         + if trophiclevel.is_carnivore(Some(Carnivore::Chasing)) {2}
           else if trophiclevel.is_herbivore(Some(Herbivore::Gathering)) {2}
           else if trophiclevel.is(TrophicLevelType::FilterFeeder) ||
                   trophiclevel.is_carnivore(Some(Carnivore::Trapping)) {-2}
           else {0}
         + if locomotion.is_immobile() {-4} else {0}
         + if reproduction.is_sexual_reproduction() {2} else {0};
        match 2.d6() + modifier {
            ..=3 => Self::NoSense(false),
            4|5 => Self::NoSense(true),
            6..=8 => Self::Normal,
            9|10 => if habitat.is_aquatic() {
                Self::AcuteTaste
            } else {
                Self::AcuteTasteAndSmell
            },
            11.. => if habitat.is_aquatic() {
                Self::DiscriminatoryTaste
            } else {
                Self::AcuteTasteAndDiscriminatorySmell
            }
        }
    }
}
