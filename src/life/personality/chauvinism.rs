use dice::DiceExt;

use crate::life::{advantages::Advantage, disadvantages::{racialintolerance::RacialIntolerance, xenophilia::Xenophilia, xenophobia::Xenophobia, Disadvantage}, quirks::Quirks, sex::{arrangement::SexualArrangement, ArrangementCheck, Reproduction}, trophiclevel::{TrophicLevel, TrophicLevelType}};

use super::{organization::SocialOrganization, Personality, PersonalityEffect, PersonalityEffectLevel};

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

impl PersonalityEffect for Chauvinism {
    fn gain(&self, personality: &Personality, trophiclevel: &TrophicLevel) -> (Vec<Box<dyn Disadvantage>>, Vec<Box<dyn Advantage>>) {
        let _ = trophiclevel;
        let mut disadvs: Vec<Box<dyn Disadvantage>> = vec![];

        match self {
            Self::Chauvinistic(3) => {
                let mut ri = false;
                if personality.empathy.level() < 1 || personality.suspicion.level() > -1 {
                    disadvs.push(Box::new(RacialIntolerance));
                    ri = true
                }
                if personality.suspicion.level() > 1 {
                    disadvs.push(Box::new(Xenophobia))
                }
                if !ri {
                    disadvs.push(Box::new(Quirks::Chauvinistic))
                }
            },
            
            Self::Chauvinistic(2) => if personality.empathy.level() < 1 || personality.suspicion.level() > -1 {
                disadvs.push(Box::new(RacialIntolerance))
            } else {
                disadvs.push(Box::new(Quirks::Chauvinistic))
            },
            
            Self::Chauvinistic(_) => if personality.empathy.level() < 0 || personality.suspicion.level() > 0 {
                disadvs.push(Box::new(RacialIntolerance))
            } else {
                disadvs.push(Box::new(Quirks::Chauvinistic))
            },
            
            Self::Undiscriminating => if personality.suspicion.level() < 0 && personality.empathy.level() > 0 {
                disadvs.push(Box::new(Xenophilia::new(9)));
            } else if personality.suspicion.level() < 0 || personality.empathy.level() > 0 {
                disadvs.push(Box::new(Xenophilia::new(12)));
            } else {
                disadvs.push(Box::new(Quirks::Undiscriminating))
            },

            Self::BroadMinded(2) => if personality.suspicion.level() < 0 && personality.empathy.level() > 0 {
                disadvs.push(Box::new(Xenophilia::new(15)));
            } else {
                disadvs.push(Box::new(Quirks::BroadMinded))
            },

            Self::BroadMinded(_) => disadvs.push(Box::new(Quirks::BroadMinded)),

            _ => ()
        }

        (disadvs, vec![])
    }
}
