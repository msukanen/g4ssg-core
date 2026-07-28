use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::life::{reproduction::{Reproduction, ReproductionStrategy}, trophics::TrophicLevel};

use super::{ZeroPivotState, zero_pivot};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Concentration {
    ShortAttentionSpan { dc: u8 },
    Distractible,
    Attentive,
    Normal,
    SingleMinded { high_pain_threshold: bool },
}

impl ZeroPivotState for Concentration {
    #[inline]
    fn zp_state(&self) -> i8 {
        match self {
            Self::ShortAttentionSpan { dc: 12 } => -2,
            Self::ShortAttentionSpan { .. } => -3,
            Self::Distractible => -1,
            Self::Normal => 0,
            Self::Attentive => 1,
            Self::SingleMinded { high_pain_threshold: false } => 2,
            Self::SingleMinded { .. } => 3
        }
    }
}

impl PartialOrd for Concentration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl Concentration {
    pub(super) fn random(trophics: TrophicLevel, reproduction: &Reproduction) -> Self {
        let mut modf = if trophics.intersects(TrophicLevel::POUNCING | TrophicLevel::CHASING) {1} else {0};
        if matches!(reproduction.strategy, ReproductionStrategy::K { strong: true, .. }) {
            modf += 1;
        }
        match zero_pivot(modf) {
            ..=-3 => Self::ShortAttentionSpan { dc: 9 },
            -2 => Self::ShortAttentionSpan { dc: 12 },
            -1 => Self::Distractible,
             0 => Self::Normal,
             1 => Self::Attentive,
             2 => Self::SingleMinded { high_pain_threshold: false },
             _ => Self::SingleMinded { high_pain_threshold: true }
        }
    }

    const fn rank(&self) -> u8 {
        match self {
            Self::ShortAttentionSpan { dc } if *dc <= 9 => 0,
            Self::ShortAttentionSpan { .. } => 1,
            Self::Distractible => 2,
            Self::Normal => 3,
            Self::Attentive => 4,
            Self::SingleMinded { high_pain_threshold: false } => 5,
            Self::SingleMinded { .. } => 6
        }
    }
}
