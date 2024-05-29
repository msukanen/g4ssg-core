use dice::DiceExt;

use crate::life::{advantages::{fearlessness::{Fearlessness, Unfazeable}, Advantage}, disadvantages::{cowardice::Cowardice, fearfulness::Fearfulness, overconfidence::Overconfidence, paranoia::Paranoia, Disadvantage}, quirks::Quirks, senses::vision::Vision, size::SizeCategory, trophiclevel::{Herbivore, TrophicLevel}};

use super::{curiosity::Curiosity, organization::SocialOrganization, PersonalityEffect, PersonalityEffectLevel};

#[derive(PartialEq, Clone, Copy)]
pub enum Suspicion {
    Fearfulness(i32),
    Careful,
    Normal,
    Fearlessness(i32)
}

impl Suspicion {
    pub fn random(curiosity: &Curiosity, size_category: &SizeCategory, trophiclevel: &TrophicLevel, vision: &Vision, social_organization: &SocialOrganization) -> Suspicion {
        let modifier
         = if trophiclevel.is_carnivore(None) { -1 }
           else if trophiclevel.is_herbivore(Some(Herbivore::Grazing)) { 1 }
           else { 0 }
         + match vision {
             Vision::Blindness(_) => 1,
             _ => 0
         } + match size_category {
             SizeCategory::Small => 1,
             SizeCategory::Large => -1,
             _ => 0
         } + match social_organization {
             SocialOrganization::Solitary |
             SocialOrganization::PairBond => 1,
             _ => 0
         };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::Fearlessness(3),
            -2 => Self::Fearlessness(2),
            -1 => Self::Fearlessness(1),
            0 => Self::Normal,
            1 => Self::Careful,
            2 => if curiosity.level() < -2 {Self::Careful} else {Self::Fearfulness(1)},
            3.. => Self::Fearfulness(2)
        }
    }

    pub fn shift_based_on(&self, curiosity: &Curiosity) -> Suspicion {
        match self {
            Self::Fearfulness(1) => if curiosity.level() <= -3 {Self::Careful} else {*self},
            Self::Careful => if curiosity.level() <= -2 {Self::Normal} else {*self},
            _ => *self
        }
    }
}

impl PersonalityEffectLevel for Suspicion {
    fn level(&self) -> i32 {
        match self {
            Self::Fearfulness(2) => -3,
            Self::Fearfulness(_) => -2,
            Self::Careful => -1,
            Self::Normal => 0,
            Self::Fearlessness(x) => *x,
        }
    }
}

impl PersonalityEffect for Suspicion {
    fn gain(&self, personality: &super::Personality, trophiclevel: &TrophicLevel) -> (Vec<Box<dyn Disadvantage>>, Vec<Box<dyn Advantage>>) {
        let mut advs: Vec<Box<dyn Advantage>> = vec![];
        let mut disadvs: Vec<Box<dyn Disadvantage>> = vec![];

        match self {
            Self::Fearfulness(2) => {
                disadvs.push(Box::new(Fearfulness::new(2)));
                if trophiclevel.is_herbivore(None) {
                    disadvs.push(Box::new(Cowardice))
                } else {
                    disadvs.push(Box::new(Paranoia))
                }
            },
            Self::Fearfulness(_) => disadvs.push(Box::new(Fearfulness::new(1))),
            Self::Careful => disadvs.push(Box::new(Quirks::Careful)),
            Self::Fearlessness(1) => advs.push(Box::new(Fearlessness::new(1))),
            Self::Fearlessness(2) => {
                advs.push(Box::new(Fearlessness::new(2)));
                if personality.egoism.level() >= 2 {
                    disadvs.push(Box::new(Overconfidence))
                }
            },
            Self::Fearlessness(3) => {
                if personality.chauvinism.level() <= -3 {
                    advs.push(Box::new(Unfazeable))
                } else {
                    advs.push(Box::new(Fearlessness::new(3)))
                }
                if personality.egoism.level() >= 1 {
                    disadvs.push(Box::new(Overconfidence))
                }
            },
            _ => ()
        }

        (disadvs, advs)
    }
}
