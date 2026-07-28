use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::life::{reproduction::*, socialorg::SocialOrganization, trophics::TrophicLevel};

use super::{ZeroPivotState, zero_pivot};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Gregariousness {
    Loner { dc: u8 },
    Uncongenial,
    Normal,
    Congenial,
    Chummy,
    Gregarious,
}

impl ZeroPivotState for Gregariousness {
    #[inline]
    fn zp_state(&self) -> i8 {
        self.rank() as i8 - 3
    }
}

impl PartialOrd for Gregariousness {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl Gregariousness {
    /// Partial ranking; [Gregariousness::Loner] is handled separately in [Gregariousness::partial_cmp].
    const fn rank(&self) -> u8 {
        match self {
            Self::Loner { dc } if *dc <= 9 => 0,
            Self::Loner { .. } => 1,
            Self::Uncongenial => 2,
            Self::Normal => 3,
            Self::Congenial => 4,
            Self::Chummy => 5,
            Self::Gregarious => 6,
        }
    }

    pub(super) fn random(
        trophics: TrophicLevel,
        reproduction: &Reproduction,
        social_organization: SocialOrganization
    ) -> Self {
        use TrophicLevel as T;

        let mut modf =
            if trophics.intersects(T::POUNCING|T::SCAVENGER|T::FILTER_FEEDER|T::AUTOTROPH|T::HERBIVORE)
                { -1 } else { 0 };
        modf += match social_organization {
            SocialOrganization::Solitary |
            SocialOrganization::PairBond => -1,
            
            SocialOrganization::MedGroup { .. } |
            SocialOrganization::LgHerd { .. } => 1,

            SocialOrganization::Hive => 2,
            
            _ => 0
        };
        modf += match reproduction.arrangement {
            SexualArrangement::Asexual       |
            SexualArrangement::Hermaphrodite => -1,
            _ => 0
        };
        if matches!(reproduction.gestation, Gestation::Spawning) {
            modf -= 1;
        }
        match zero_pivot(modf) {
            ..=-3 => Self::Loner { dc: 9 },
            -2 => Self::Loner { dc: 12 },
            -1 => Self::Uncongenial,
             0 => Self::Normal,
             1 => Self::Congenial,
             2 => Self::Chummy,
             _ => Self::Gregarious
        }
    }
}
