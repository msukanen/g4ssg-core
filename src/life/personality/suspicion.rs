use dice::DiceExt;

use crate::life::{senses::vision::Vision, size::SizeCategory, trophiclevel::{Herbivore, TrophicLevel}};

use super::{curiosity::Curiosity, egoism::Egoism, organization::SocialOrganization, PersonalityEffectLevel};

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
