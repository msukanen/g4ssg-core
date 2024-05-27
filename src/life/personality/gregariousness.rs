use dice::DiceExt;

use crate::life::{sex::{arrangement::SexualArrangement, gestation::Gestation, ArrangementCheck, Reproduction}, trophiclevel::{Carnivore, TrophicLevel, TrophicLevelType}};

use super::{organization::SocialOrganization, PersonalityEffectLevel};

pub enum Gregariousness {
    Gregarious,
    Chummy,
    Congenial,
    Normal,
    Uncongenial,
    Loner(i32),
}

impl Gregariousness {
    pub fn random(
        trophiclevel: &TrophicLevel,
        reproduction: &Reproduction,
        social_organization: &SocialOrganization
    ) -> Gregariousness {
        let modifier
         = if trophiclevel.is_carnivore(Some(Carnivore::Pouncing))
           || trophiclevel.is(TrophicLevelType::Scavenger)
           || trophiclevel.is_autotroph()
           || trophiclevel.is(TrophicLevelType::FilterFeeder)
           || trophiclevel.is_herbivore(None) { -1 } else { 0 }
         + match social_organization {
             SocialOrganization::Solitary         |
             SocialOrganization::PairBond         => -1,
             SocialOrganization::MediumGroup(_,_) |
             SocialOrganization::LargeHerd(_)     => 1,
             SocialOrganization::Hive             => 2,
             _ => 0
         } + if reproduction.is(&SexualArrangement::Hermaphrodite)
             || reproduction.is(&SexualArrangement::Asexual) { -1 }
             else { 0 }
          + match reproduction.gestation() {
              Gestation::Spawning(_) => -1,
              _ => 0
          };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::Loner(9),
            -2 => Self::Loner(12),
            -1 => Self::Uncongenial,
            0 => Self::Normal,
            1 => Self::Congenial,
            2 => Self::Chummy,
            3.. => Self::Gregarious
        }
    }
}

impl PersonalityEffectLevel for Gregariousness {
    fn level(&self) -> i32 {
        match self {
            Self::Gregarious => 3,
            Self::Chummy => 2,
            Self::Congenial => 1,
            Self::Normal => 0,
            Self::Uncongenial => -1,
            Self::Loner(12) => -2,
            Self::Loner(_) => -3,
            _ => panic!("Coder error detected! No matching branch for Gregariousness level() function!")
        }
    }
}
