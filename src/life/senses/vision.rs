use dicebag::{DiceExt, HiLo};

use crate::life::{habitat::{water::WaterHabitat, Habitat}, locomotion::{Locomotion, LocomotionMode}, trophiclevel::{Herbivore, TrophicLevel, TrophicLevelType}};

pub enum Vision {
    Blindness(bool),
    Colorblindness,
    BadSight(bool),
    Normal,
    Telescopic
}

impl Vision {
    pub fn random(primary_sense: bool, habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> Vision {
        let modifier
        = if locomotion.primary_match(LocomotionMode::Digging)
             || locomotion.is_immobile() {-4}
          else if locomotion.is_climber() {2}
          else {0}
        + match habitat {Habitat::Water(WaterHabitat::DeepOceanVents) => -4, _=> 0}
        + if trophiclevel.is_carnivore(None)
            || trophiclevel.is(TrophicLevelType::Herbivore(Herbivore::Gathering)) {2}
          else if trophiclevel.is(TrophicLevelType::FilterFeeder) {-2}
          else {0}
        + if primary_sense {4} else {0};
        
        if habitat.is_space() {
            match 3.d6() + modifier {
                ..=9 => Self::Blindness(false),
                10|11 => if 1.d2().lo() {Self::BadSight(false)} else {Self::Colorblindness},
                12..=14 => Self::Normal,
                15.. => Self::Telescopic
            }
        } else {
            match 3.d6() + modifier {
                ..=6 => Self::Blindness(false),
                7 => Self::Blindness(true),
                8|9 => Self::BadSight(true),
                10|11 => if 1.d2().lo() {Self::BadSight(false)} else {Self::Colorblindness},
                12..=14 => Self::Normal,
                15.. => Self::Telescopic
            }
        }
    }

    pub fn is_blind(&self, completely_blind: bool) -> bool {
        match self {
            Self::Blindness(true) => if completely_blind {false} else {true},
            Self::Blindness(_) => true,
            _ => false
        }
    }
}
