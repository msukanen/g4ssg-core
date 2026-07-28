//! Chauvinism — how the species views itself as a group compared to others.

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::{ZeroPivotState, zero_pivot, Empathy, Suspicion};
use crate::life::{reproduction::{Gestation, Reproduction, SexualArrangement}, socialorg::SocialOrganization, trophics::TrophicLevel};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Chauvinism {
    Xenophilia { dc: u8 },
    Undiscriminating,
    BroadMinded { v_: u8 },
    Normal,
    Chauvinistic { v_: u8 },
    RacialIntolerance,
    Xenophobia,
}

impl PartialOrd for Chauvinism {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl Chauvinism {
    #[inline]
    const fn rank(&self) -> i8 {
        match self {
            Self::Xenophilia { dc } if *dc <= 9 => 0,
            Self::Xenophilia { dc } if *dc <= 12 => 1,
            Self::Xenophilia { .. } => 2,
            Self::Undiscriminating => 3,
            Self::BroadMinded { .. } => 4,
            Self::Normal => 5,
            Self::Chauvinistic { .. } => 6,
            Self::RacialIntolerance => 7,
            Self::Xenophobia => 8
        }
    }

    pub(super) fn random(
        trophics: TrophicLevel,
        reproduction: &Reproduction,
        social_organization: SocialOrganization,
    ) -> Self {
        let mut modf =
            if trophics.intersects(TrophicLevel::AUTOTROPH | TrophicLevel::FILTER_FEEDER)
                { -1 }
            else if trophics.intersects(TrophicLevel::PARASITE | TrophicLevel::SCAVENGER)
                { -2 }
            else { 0 };
        modf += match social_organization {
            SocialOrganization::Solitary |
            SocialOrganization::PairBond => -1,
            SocialOrganization::Hive              |
            SocialOrganization::SmallGroup { .. } |
            SocialOrganization::MedGroup { .. }   => 2,
            _ => 0
        };
        if matches!(reproduction.gestation, Gestation::Spawning) ||
           matches!(reproduction.arrangement, SexualArrangement::Asexual)
        {
            modf -= 2;
        }
        match zero_pivot(modf) {
            ..=-3 => Self::Undiscriminating,
            -2 => Self::BroadMinded { v_: 1 },
            -1 => Self::BroadMinded { v_: 0 },
             0 => Self::Normal,
             1 => Self::Chauvinistic { v_: 0 },
             2 => Self::Chauvinistic { v_: 1 },
             _ => Self::Chauvinistic { v_: 2 },
        }

    }

    pub fn adjust_by_empsusp(&mut self, e: Empathy, s: Suspicion) {
        match (*self, e.zp_state(), s.zp_state()) {
            (Self::Chauvinistic { v_: 2 }, _, 2..)   => *self = Self::Xenophobia,
            (Self::Chauvinistic { v_: 2 }, ..=0, _)  |
            (Self::Chauvinistic { v_: 2 }, _, 0..)   => *self = Self::RacialIntolerance,
            (Self::Chauvinistic { v_: 1 }, ..=0, _)  |
            (Self::Chauvinistic { v_: 1 }, _, 0..)   => *self = Self::RacialIntolerance,
            (Self::Chauvinistic { v_: 0 }, ..=-1, _) |
            (Self::Chauvinistic { v_: 0 }, _, 1..)   => *self = Self::RacialIntolerance,
            
            (Self::BroadMinded { v_: 1 }, 1.., ..=-1) => *self = Self::Xenophilia { dc: 15 },
            
            (Self::Undiscriminating, -1.., ..=-1) => *self = Self::Xenophilia { dc: 9 },
            (Self::Undiscriminating, -1.., _)     |
            (Self::Undiscriminating, _, ..=-1)    => *self = Self::Xenophilia { dc: 12 },

            _ => ()
        }
    }
}

impl ZeroPivotState for Chauvinism {
    #[inline]
    fn zp_state(&self) -> i8 {
        match self {
            Self::Xenophobia           |
            Self::RacialIntolerance    |
            Self::Chauvinistic { v_: 2 } => 3,
            Self::Chauvinistic { v_: 1 } => 2,
            Self::Chauvinistic { v_: _ } => 1,
            Self::Normal => 0,
            Self::BroadMinded { v_: 0 } => -1,
            Self::BroadMinded { v_: _ } => -2,
            Self::Undiscriminating  |
            Self::Xenophilia { .. } => -3,
        }
    }
}