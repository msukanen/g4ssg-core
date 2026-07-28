use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::life::{reproduction::{Reproduction, ReproductionStrategy}, socialorg::SocialOrganization, trophics::TrophicLevel};

use super::{ZeroPivotState, zero_pivot, Gregariousness, Suspicion};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Empathy {
    Empathetic { charitable: bool },
    Sensitive,
    Responsive,
    Normal,
    Oblivious,
    Callous,
    Low { bloodlust: bool },
}

impl Empathy {
    const fn rank(&self) -> u8 {
        match self {
            Self::Low { bloodlust: true } => 0,
            Self::Low { .. } => 1,
            Self::Callous => 2,
            Self::Oblivious => 3,
            Self::Normal => 4,
            Self::Responsive => 5,
            Self::Sensitive => 6,
            Self::Empathetic { charitable: false } => 7,
            Self::Empathetic { .. } => 8
        }
    }

    pub(super) fn random(
        trophics: TrophicLevel,
        reproduction: &Reproduction,
        social_organization: SocialOrganization
    ) -> Self {
        let mut modf = match reproduction.strategy {
            ReproductionStrategy::K { strong: true, .. } => 1,
            _ => 0
        };
        modf += match social_organization {
            SocialOrganization::PairBond |
            SocialOrganization::Solitary => -1,
            SocialOrganization::SmallGroup { .. } |
            SocialOrganization::MedGroup { .. } => 1,
            _ => 0
        };
        modf +=
            if trophics.contains(TrophicLevel::CHASING)
                { 1 }
            else if trophics.intersects(
                TrophicLevel::AUTOTROPH |
                TrophicLevel::FILTER_FEEDER |
                TrophicLevel::GRAZING |
                TrophicLevel::SCAVENGER)
                { -1 }
            else { 0 };
        let bloodlust = trophics.contains(TrophicLevel::CARNIVORE);
        match zero_pivot(modf) {
            ..=-3 => Self::Low { bloodlust },
            -2 => Self::Callous,
            -1 => Self::Oblivious,
             0 => Self::Normal,
             1 => Self::Responsive,
             2 => Self::Sensitive,
             _ => Self::Empathetic { charitable: false }
        }
    }

    pub fn adjust_by_gregsusp(&mut self, g: Gregariousness, s: Suspicion) {
        match (*self, g.zp_state(), s.zp_state()) {
            (Self::Empathetic { .. }, 1.., _) => *self = Self::Empathetic { charitable: true },
            (Self::Responsive, 1.., ..=-1) => *self = Self::Sensitive,
            _ => ()
        }
    }
}

impl PartialOrd for Empathy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl ZeroPivotState for Empathy {
    #[inline]
    fn zp_state(&self) -> i8 {
        match self {
            Self::Empathetic { .. } => 3,
            Self::Sensitive => 2,
            Self::Responsive => 1,
            Self::Normal => 0,
            Self::Oblivious => -1,
            Self::Callous => -2,
            Self::Low { .. } => -3
        }
    }
}