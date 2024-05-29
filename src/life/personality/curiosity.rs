use dice::DiceExt;

use crate::life::{advantages::Advantage, disadvantages::{curious::Curious, incurious::Incurious, Disadvantage}, quirks::Quirks, senses::{vision::Vision, Senses}, sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::{Herbivore, TrophicLevel, TrophicLevelType}};

use super::{concentration::Concentration, suspicion::Suspicion, Personality, PersonalityEffect, PersonalityEffectLevel};

#[derive(PartialEq, Clone, Copy)]
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
             &Vision::Blindness(_) => -1,
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

    pub fn shift_based_on(&self, suspicion: &Suspicion, concentration: &Concentration) -> Curiosity {
        if concentration.level() <= 0 {
            match self {
                Self::Curious(12) => Self::Curious(9),
                Self::Curious(9) => Self::Curious(6),
                Self::Nosy => Self::Curious(12),
                _ => *self
            }
        } else if suspicion.level() <= 0 {
            match self {
                Self::Curious(9) => Self::Curious(6),
                Self::Incurious(12) => Self::Incurious(9),
                _ => *self
            }
        } else {
            *self
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

impl PersonalityEffect for Curiosity {
    fn gain(&self, personality: &Personality, trophiclevel: &TrophicLevel) -> (Vec<Box<dyn Disadvantage>>, Vec<Box<dyn Advantage>>) {
        let _ = trophiclevel;
        let _ = personality;
        let mut disadvs: Vec<Box<dyn Disadvantage>> = vec![];
        match self {
            Self::Curious(control) => disadvs.push(Box::new(Curious::new(*control, 1))),
            Self::Nosy => disadvs.push(Box::new(Quirks::Nosy)),
            Self::Staid => disadvs.push(Box::new(Quirks::Staid)),
            Self::Incurious(control) => disadvs.push(Box::new(Incurious::new(*control))),
            _ => ()
        }
        (disadvs, vec![])
    }
}
