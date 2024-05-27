use dice::DiceExt;

use crate::life::{sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::{Carnivore, TrophicLevel}};

use super::PersonalityEffectLevel;

pub enum Concentration {
    ShortAttentionSpan(i32),
    Distractible,
    Normal,
    Attentive,
    SingleMinded(i32)
}

impl Concentration {
    pub fn random(trophiclevel: &TrophicLevel, reproduction: &Reproduction) -> Concentration {
        let modifier
         = if trophiclevel.is_carnivore(Some(Carnivore::Pouncing))
           || trophiclevel.is_carnivore(Some(Carnivore::Chasing))
           { 1 }
           else if trophiclevel.is_herbivore(None) { -1 } else { 0 }
         + match reproduction.strategy() {
             ReproductionStrategy::StrongK(_) => 1,
             _ => 0
         };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::ShortAttentionSpan(9),
            -2 => Self::ShortAttentionSpan(12),
            -1 => Self::Distractible,
            0 => Self::Normal,
            1 => Self::Attentive,
            2 => Self::SingleMinded(1),
            3.. => Self::SingleMinded(2)
        }
    }
}

impl PersonalityEffectLevel for Concentration {
    fn level(&self) -> i32 {
        match self {
            Self::ShortAttentionSpan(9) => -3,
            Self::ShortAttentionSpan(_) => -2,
            Self::Distractible => -1,
            Self::Normal => 0,
            Self::Attentive => 1,
            Self::SingleMinded(a) => *a + 1,
        }
    }
}
