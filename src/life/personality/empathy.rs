use dice::DiceExt;

use crate::life::{sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::{Carnivore, Herbivore, TrophicLevel, TrophicLevelType}};

use super::{organization::SocialOrganization, PersonalityEffectLevel};

pub enum Empathy {
    Empathy(bool),
    Sensitive,
    Responsive,
    Normal,
    Oblivious,
    Callous,
}

impl Empathy {
    pub fn random(trophiclevel: &TrophicLevel, social_organization: &SocialOrganization, reproduction: &Reproduction) -> Empathy {
        let modifier
         = if trophiclevel.is_autotroph()
           || trophiclevel.is(TrophicLevelType::FilterFeeder)
           || trophiclevel.is_herbivore(Some(Herbivore::Grazing))
           || trophiclevel.is(TrophicLevelType::Scavenger) { -1 }
           else if trophiclevel.is_carnivore(Some(Carnivore::Chasing)) { 1 }
           else { 0 }
         + match social_organization {
             SocialOrganization::Solitary |
             SocialOrganization::PairBond => -1,
             SocialOrganization::MediumGroup(_,_) |
             SocialOrganization::SmallGroup(_,_)  => 1,
             _ => 0
         } + match reproduction.strategy() {
             ReproductionStrategy::StrongK(_) => 1,
             _ => 0
         };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::Empathy(true),
            -2 => Self::Callous,
            -1 => Self::Oblivious,
            0 => Self::Normal,
            1 => Self::Responsive,
            2 => Self::Sensitive,
            3.. => Self::Empathy(false)
        }
    }
}

impl PersonalityEffectLevel for Empathy {
    fn level(&self) -> i32 {
        match self {
            Self::Empathy(true) => -3,
            Self::Empathy(_)    =>  3,
            Self::Normal     =>  0,
            Self::Oblivious  => -1,
            Self::Responsive =>  1,
            Self::Callous    => -2,
            Self::Sensitive  =>  2
        }
    }
}
