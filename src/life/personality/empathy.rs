use dicebag::DiceExt;

use crate::life::{advantages::{charitable::Charitable, empathy::Sensitive, Advantage}, disadvantages::{bloodlust::Bloodlust, callous::Callous, oblivious::Oblivious, Disadvantage}, sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::{Carnivore, Herbivore, TrophicLevel, TrophicLevelType}, quirks::Quirks};

use super::{gregariousness::Gregariousness, organization::SocialOrganization, suspicion::Suspicion, Personality, PersonalityEffect, PersonalityEffectLevel};

#[derive(PartialEq, Clone, Copy)]
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

    pub fn shift_based_on(&self, gregariousness: &Gregariousness, suspicion: &Suspicion) -> Empathy {
        match self {
            Self::Responsive => if gregariousness.level() > 0 && suspicion.level() < 0 {Self::Sensitive} else {*self},
            _ => *self
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

impl PersonalityEffect for Empathy {
    fn gain(&self, personality: &Personality, trophiclevel: &TrophicLevel) -> (Vec<Box<dyn Disadvantage>>, Vec<Box<dyn Advantage>>) {
        let mut disadvs: Vec<Box<dyn Disadvantage>> = vec![];
        let mut advs: Vec<Box<dyn Advantage>> = vec![];

        match self {
            Self::Empathy(false) => {
                advs.push(Box::new(crate::life::advantages::empathy::Empathy));
                if personality.gregariousness.level() > 0 {
                    advs.push(Box::new(Charitable));
                }
            },
            Self::Empathy(true) => {
                advs.push(Box::new(Sensitive));
                if trophiclevel.is_carnivore(None) {
                    disadvs.push(Box::new(Bloodlust));
                }
            },
            Self::Responsive => disadvs.push(Box::new(Quirks::Responsive)),
            Self::Oblivious => disadvs.push(Box::new(Oblivious)),
            Self::Callous => disadvs.push(Box::new(Callous)),
            _ => ()
        }

        (disadvs, advs)
    }
}
