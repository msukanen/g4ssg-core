use std::cmp::Ordering;

use dicebag::lo;
use serde::{Deserialize, Serialize};

use crate::life::{intelligence::Intelligence, reproduction::{Reproduction, ReproductionStrategy}, socialorg::SocialOrganization};

use super::{ZeroPivotState, zero_pivot, Suspicion};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Playfulness {
    NoSenseOfHumor,
    DeadSerious,
    Serious,
    /// Playful, but not quite Human-playful level so.
    Normal,
    /// "Humans" are more playful than avg. Normal and thus in this particular case aren't baseline 'Normal'.
    Playful,
    CompulsivePlayfulness { dc: u8 },
    Trickster,
}

impl ZeroPivotState for Playfulness {
    #[inline]
    fn zp_state(&self) -> i8 {
        (self.rank() as i8).min(6) - 3
    }
}

impl PartialOrd for Playfulness {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl Playfulness {
    #[inline]
    const fn rank(&self) -> u8 {
        match self {
            Self::NoSenseOfHumor => 0,
            Self::DeadSerious => 1,
            Self::Serious => 2,
            Self::Normal => 3,
            Self::Playful => 4,
            Self::CompulsivePlayfulness { dc } if *dc >= 12 => 5,
            Self::CompulsivePlayfulness { .. } => 6,
            Self::Trickster => 7
        }
    }

    pub(super) fn random(
        reproduction: &Reproduction,
        intelligence: Option<Intelligence>,
        social_organization: SocialOrganization
    ) -> Self {
        let Some(iq) = intelligence else { return Self::NoSenseOfHumor };

        let mut modf = match iq {
            Intelligence::Preprogrammed => return if lo!() { Self::NoSenseOfHumor } else { Self::DeadSerious },
            Intelligence::Low { iq }        |
            Intelligence::High { iq }       |
            Intelligence::Presapient { iq } => if iq >= 2 {1} else {0},
            Intelligence::Sapient { .. } => 1
        };
        modf += match reproduction.strategy {
            ReproductionStrategy::K { strong: true, .. } => 2,
            ReproductionStrategy::K { .. } => 1,
            _ => 0
        };
        if matches!(social_organization, SocialOrganization::Solitary) {
            modf -= 1;
        }
        match zero_pivot(modf) {
            ..=-3 => Self::NoSenseOfHumor,
            -2 => Self::DeadSerious,
            -1 => Self::Serious,
             0 => Self::Normal,
             1 => Self::Playful,
             2 => Self::CompulsivePlayfulness { dc: 12 },
             _ => Self::CompulsivePlayfulness { dc: 9 }
        }
    }

    pub fn adjust_by_suspicion(&mut self, s: Suspicion) {
        match (*self, s) {
            (Self::CompulsivePlayfulness { dc: 9 }, Suspicion::Fearlessness { overconfidence: true,.. })
                => *self = Self::Trickster,
            _ => ()
        }
    }
}
