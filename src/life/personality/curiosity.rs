use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use crate::life::{personality::concentration::Concentration, reproduction::{Reproduction, ReproductionStrategy}, senses::{Senses, vision::Vision}, trophics::TrophicLevel};

use super::{ZeroPivotState, zero_pivot};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Curiosity {
    Incurious { dc: u8 },
    Staid,
    Normal,
    Nosy,
    Curious { dc: u8 },
}

impl ZeroPivotState for Curiosity {
    #[inline]
    fn zp_state(&self) -> i8 {
        self.rank() as i8 - 3
    }
}

impl PartialOrd for Curiosity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl Curiosity {
    const fn rank(&self) -> u8 {
        match self {
            Self::Incurious { dc } if *dc <= 9 => 0,
            Self::Incurious { .. } => 1,
            Self::Staid => 2,
            Self::Normal => 3,
            Self::Nosy => 4,
            Self::Curious { dc } if *dc >= 12 => 5,
            Self::Curious { .. } => 6
        }
    }

    pub(super) fn random(
        trophics: TrophicLevel,
        senses: &Senses,
        reproduction: &Reproduction,
        concentration: Concentration
    ) -> Self {
        let mut modf =
            if trophics.contains(TrophicLevel::OMNIVORE)
                { 1 }
            else if trophics.intersects(TrophicLevel::GRAZING | TrophicLevel::FILTER_FEEDER)
                { -1 }
            else { 0 };
        modf += match senses.vision {
            None |
            Some(Vision::BadSight { .. }) => -1,
            _ => 0
        };
        modf += match reproduction.strategy {
            ReproductionStrategy::K { strong: true, .. } => 1,
            ReproductionStrategy::R { strong: true, .. } => -1,
            _ => 0
        };
        let mut lvl = match zero_pivot(modf) {
            ..=-3 => return Self::Incurious { dc: 9 },
            -2 => return Self::Incurious { dc: 12 },
            -1 => return Self::Staid,
             0 => return Self::Normal,
             1 => return Self::Nosy,
             2 => Self::Curious { dc: 12 },
             _ => Self::Curious { dc: 9 }
        };
        // amp curiosity if norm. or worse concentration
        match concentration {
            Concentration::Normal |
            Concentration::Distractible |
            Concentration::ShortAttentionSpan { .. } => match &mut lvl {
                Curiosity::Curious { dc } => *dc -= 3,
                _ => ()
            },
            _ => ()
        }
        
        lvl
    }
}
