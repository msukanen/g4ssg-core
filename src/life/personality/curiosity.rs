use dice::DiceExt;

use crate::life::{senses::{vision::Vision, Senses}, sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::{Herbivore, TrophicLevel, TrophicLevelType}};

use super::PersonalityEffectLevel;

pub enum Curiosity {
    Curious(i32),
    Nosy,
    Normal,
    Staid,
    Incurious(i32)
}

impl Curiosity {
    pub fn random(trophiclevel: &TrophicLevel, senses: &Senses, reproduction: &Reproduction) -> Curiosity {
        let modifier
         = if trophiclevel.is(TrophicLevelType::Omnivore) { 1 }
           else if trophiclevel.is_herbivore(Some(Herbivore::Grazing))
                || trophiclevel.is(TrophicLevelType::FilterFeeder) { -1 } else { 0 }
         + match senses.vision() {
             Vision::Blindness(_) => -1,
             _ => 0}
         + match reproduction.strategy() {
             ReproductionStrategy::StrongR(_) => -1,
             ReproductionStrategy::StrongK(_) => 1,
             _ => 0
         };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::Incurious(9),
            -2 => Self::Incurious(12),
            -1 => Self::Staid,
            0 => Self::Normal,
            1 => Self::Nosy,
            2 => Self::Curious(12),
            3.. => Self::Curious(9)
        }
    }
}

impl PersonalityEffectLevel for Curiosity {
    fn level(&self) -> i32 {
        match self {
            Self::Incurious(9) => -3,
            Self::Incurious(_) => -2,
            Self::Staid => -1,
            Self::Normal => 0,
            Self::Nosy => 1,
            Self::Curious(12) => 2,
            Self::Curious(_) => 3,
        }
    }
}