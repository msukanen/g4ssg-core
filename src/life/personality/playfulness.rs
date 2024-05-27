use dice::DiceExt;

use crate::life::{intelligence::Intelligence, sex::{reprstrategy::ReproductionStrategy, Reproduction}};

use super::{organization::SocialOrganization, PersonalityEffectLevel};

pub enum Playfulness {
    Compulsive(i32),
    Playful,
    Normal,
    Serious,
    ORHWetBlanket,
    NoSenseOfHumor
}

impl Playfulness {
    pub fn random(reproduction: &Reproduction, social_organization: &SocialOrganization, intelligence: &Intelligence) -> Playfulness {
        let modifier
         = match reproduction.strategy() {
             ReproductionStrategy::ModerateK(_) => 1,
             ReproductionStrategy::StrongK(_) => 2,
             _ => 0
         } + match intelligence {
            Intelligence::BestialHighIQ(_) |
            Intelligence::BestialLowIQ(3)  |
            Intelligence::Presapient       |
            Intelligence::Sapient(_)       =>  1,
            Intelligence::Preprogrammed    => -3,
            _ => 0
         } + if social_organization == &SocialOrganization::Solitary { -1 } else { 0 };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::NoSenseOfHumor,
            -2 => Self::ORHWetBlanket,
            -1 => Self::Serious,
            0 => Self::Normal,
            1 => Self::Playful,
            2 => Self::Compulsive(12),
            3.. => Self::Compulsive(9)
        }
    }
}

impl PersonalityEffectLevel for Playfulness {
    fn level(&self) -> i32 {
        match self {
            Self::NoSenseOfHumor => -3,
            Self::ORHWetBlanket => -2,
            Self::Serious => -1,
            Self::Normal => 0,
            Self::Playful => 1,
            Self::Compulsive(12) => 2,
            Self::Compulsive(_) => 3
        }
    }
}
