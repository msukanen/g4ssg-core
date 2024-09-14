use dicebag::DiceExt;

use crate::life::{advantages::Advantage, disadvantages::{nosenseofhumor::NoSenseOfHumor, orh::wetblanket::WetBlanket, playfulness::CompulsivePlayfulness, trickster::Trickster, Disadvantage}, intelligence::Intelligence, sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::TrophicLevel, quirks::Quirks};

use super::{organization::SocialOrganization, Personality, PersonalityEffect, PersonalityEffectLevel};

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

impl PersonalityEffect for Playfulness {
    fn gain(&self, personality: &Personality, trophiclevel: &TrophicLevel) -> (Vec<Box<dyn Disadvantage>>, Vec<Box<dyn Advantage>>) {
        let _ = trophiclevel;
        let mut disadvs: Vec<Box<dyn Disadvantage>> = vec![];

        match self {
            Self::Compulsive(9) => if personality.overconfident {
                disadvs.push(Box::new(Trickster))
            } else {
                disadvs.push(Box::new(CompulsivePlayfulness::new(9)))
            },
            Self::Compulsive(12) => disadvs.push(Box::new(CompulsivePlayfulness::new(12))),
            Self::Playful => disadvs.push(Box::new(Quirks::Playful)),
            Self::Serious => disadvs.push(Box::new(Quirks::Serious)),
            Self::ORHWetBlanket => disadvs.push(Box::new(WetBlanket)),
            Self::NoSenseOfHumor => disadvs.push(Box::new(NoSenseOfHumor)),
            _ => ()
        }

        (disadvs, vec![])
    }
}
