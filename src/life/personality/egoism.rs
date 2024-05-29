use dice::DiceExt;

use crate::life::{advantages::{selfless::Selfless, Advantage}, disadvantages::{selfish::Selfish, Disadvantage}, quirks::Quirks, sex::mating::MatingBehavior};

use super::{chauvinism::Chauvinism, empathy::Empathy, organization::SocialOrganization, suspicion::Suspicion, PersonalityEffect, PersonalityEffectLevel};

#[derive(PartialEq, Clone, Copy)]
pub enum Egoism {
    Selfish(i32),
    Proud,
    Normal,
    Humble,
    Selfless(i32)
}

impl Egoism {
    pub fn random(chauvinism: &Chauvinism, empathy: &Empathy, suspicion: &Suspicion, mating_behavior: &MatingBehavior, organization: &SocialOrganization) -> Egoism {
        let modifier
         = if organization == &SocialOrganization::Solitary { 1 }
           else if organization == &SocialOrganization::Hive { -1 }
           else { 0 }
         + if mating_behavior == &MatingBehavior::Harem { 1 } else { 0 };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::Selfless(6),
            -2 => Self::Selfless(if chauvinism.level() > 1 {9} else {12}),
            -1 => Self::Humble,
            0 => Self::Normal,
            1 => Self::Proud,
            2 => Self::Selfish(if suspicion.level() > 1 || empathy.level() < 0 { 9 } else { 12 }),
            3.. => Self::Selfish(9)
        }
    }

    pub fn shift_based_on(&self, suspicion: &Suspicion, chauvinism: &Chauvinism, empathy: &Empathy) -> Egoism {
        match self {
            Self::Selfish(12) => if suspicion.level() > 0 || empathy.level() < 0 {Self::Selfish(9)} else {*self},
            Self::Proud => if suspicion.level() >= 2 || empathy.level() <= -2 {Self::Selfish(9)}
                           else if suspicion.level() > 0 {Self::Selfish(12)}
                           else {*self}
            Self::Selfless(12) => if chauvinism.level() >= 2 {Self::Selfless(9)} else {*self},
            _ => *self
        }
    }
}

impl PersonalityEffectLevel for Egoism {
    fn level(&self) -> i32 {
        match self {
            Self::Normal => 0,
            Self::Proud => 1,
            Self::Humble => -1,
            Self::Selfish(12) => 2,
            Self::Selfish(_) => 3,
            Self::Selfless(6) => -3,
            Self::Selfless(_) => -2
        }
    }
}

impl PersonalityEffect for Egoism {
    fn gain(&self, personality: &super::Personality, trophiclevel: &crate::life::trophiclevel::TrophicLevel) -> (Vec<Box<dyn Disadvantage>>, Vec<Box<dyn Advantage>>) {
        let _ = trophiclevel;
        let _ = personality;
        let mut disadvs: Vec<Box<dyn Disadvantage>> = vec![];
        let mut advs: Vec<Box<dyn Advantage>> = vec![];

        match self {
            Self::Selfish(control) => disadvs.push(Box::new(Selfish::new(*control))),
            Self::Proud => disadvs.push(Box::new(Quirks::Proud)),
            Self::Humble => disadvs.push(Box::new(Quirks::Humble)),
            Self::Selfless(control) => advs.push(Box::new(Selfless::new(*control))),
            _ => ()
        }

        (disadvs, advs)
    }
}
