use dice::DiceExt;

use crate::life::{sex::{arrangement::SexualArrangement, ArrangementCheck, Reproduction}, trophiclevel::{TrophicLevel, TrophicLevelType}};

use super::{organization::SocialOrganization, PersonalityEffectLevel};

pub enum Chauvinism {
    Chauvinistic(i32),
    Normal,
    BroadMinded(i32),
    Undiscriminating,
}

impl Chauvinism {
    pub fn random(trophiclevel: &TrophicLevel, social_organization: &SocialOrganization, reproduction: &Reproduction) -> Chauvinism {
        let modifier
         = if trophiclevel.is_autotroph()
           || trophiclevel.is(TrophicLevelType::FilterFeeder)
           { -1 }
           else if trophiclevel.is(TrophicLevelType::Parasite)
           || trophiclevel.is(TrophicLevelType::Scavenger)
           { -2 } else { 0 }
         + match social_organization {
             SocialOrganization::SmallGroup(_,_) |
             SocialOrganization::MediumGroup(_,_)|
             SocialOrganization::Hive            => 2,
             SocialOrganization::Solitary |
             SocialOrganization::PairBond => -1,
             _ => 0}
         + if reproduction.is(&SexualArrangement::Asexual) { -1 } else { 0 }
         + if reproduction.gestation().is_spawning() { -1 } else { 0 };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::Undiscriminating,
            -2 => Self::BroadMinded(2),
            -1 => Self::BroadMinded(1),
            0 => Self::Normal,
            1 => Self::Chauvinistic(1),
            2 => Self::Chauvinistic(2),
            3.. => Self::Chauvinistic(3),
        }
    }
}

impl PersonalityEffectLevel for Chauvinism {
    fn level(&self) -> i32 {
        match self {
            Self::Undiscriminating => -3,
            Self::BroadMinded(1) => -1,
            Self::BroadMinded(_) => -2,
            Self::Normal => 0,
            Self::Chauvinistic(x) => *x,
        }
    }
}
