use dice::DiceExt;

use crate::life::sex::mating::MatingBehavior;

use super::{chauvinism::Chauvinism, organization::SocialOrganization, PersonalityEffectLevel, suspicion::Suspicion, empathy::Empathy};

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
