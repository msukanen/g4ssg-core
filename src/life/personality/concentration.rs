use dicebag::DiceExt;

use crate::life::{advantages::{highpainthreshold::HighPainThreshold, singleminded::SingleMinded, Advantage}, disadvantages::{shortattspan::ShortAttentionSpan, Disadvantage}, quirks::Quirks, sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::{Carnivore, TrophicLevel}};

use super::{PersonalityEffect, PersonalityEffectLevel};

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

impl PersonalityEffect for Concentration {
    fn gain(&self, personality: &super::Personality, trophiclevel: &TrophicLevel) -> (Vec<Box<dyn Disadvantage>>, Vec<Box<dyn Advantage>>) {
        let _ = trophiclevel;
        let _ = personality;
        let mut advs: Vec<Box<dyn Advantage>> = vec![];
        let mut disadvs: Vec<Box<dyn Disadvantage>> = vec![];
        match self {
            Self::SingleMinded(1) => advs.push(Box::new(SingleMinded)),
            Self::SingleMinded(_) => {
                advs.push(Box::new(SingleMinded));
                advs.push(Box::new(HighPainThreshold))
            },
            Self::Attentive => disadvs.push(Box::new(Quirks::Attentive)),
            Self::Distractible => disadvs.push(Box::new(Quirks::Distractible)),
            Self::ShortAttentionSpan(control) => disadvs.push(Box::new(ShortAttentionSpan::new(*control))),
            _ => ()
        }
        (disadvs, advs)
    }
}
