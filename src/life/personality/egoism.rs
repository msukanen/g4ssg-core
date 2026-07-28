//! Egoism — a measure of how personally self-important members of this species are.
//! Individual counterpart to species Chauvinism.
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::life::{reproduction::{MatingBehavior, Reproduction, ReproductionStrategy}, socialorg::SocialOrganization};

use super::{ZeroPivotState, zero_pivot, Chauvinism, Empathy, Suspicion};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Egoism {
    Selfish { dc: u8 },
    Proud,
    Normal,
    Humble,
    Selfless { dc: u8 }
}

impl PartialOrd for Egoism {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl ZeroPivotState for Egoism {
    #[inline]
    fn zp_state(&self) -> i8 {
        match self {
            Self::Selfish { dc: 1 } => -2,
            Self::Selfish { .. } => -3,
            Self::Proud => -1,
            Self::Normal => 0,
            Self::Humble => 1,
            Self::Selfless { dc } => (1 + *dc).min(3) as i8
        }
    }
}

impl Egoism {
    #[inline]
    const fn rank(&self) -> u8 {
        match self {
            Self::Selfless { dc } if *dc <= 6 => 0,
            Self::Selfless { dc } if *dc <= 9 => 1,
            Self::Selfless { .. } => 2,
            Self::Humble => 3,
            Self::Normal => 4,
            Self::Proud => 5,
            Self::Selfish { dc } if *dc >= 12 => 6,
            Self::Selfish { .. } => 7
        }
    }

    /// Generate random [Egoism] value, which may be different for female/male.
    /// 
    /// # Returns
    /// - male / female [Egoism] value.
    pub(super) fn random(
        reproduction: &Reproduction,
        mating: MatingBehavior,
        social_organization: SocialOrganization
    ) -> (Self, Self) {
        const fn ego(r: i8) -> Egoism {
            match r {
                ..=-3 => Egoism::Selfish { dc: 9 },
                -2 => Egoism::Selfish { dc: 12 },
                -1 => Egoism::Proud,
                 0 => Egoism::Normal,
                 1 => Egoism::Humble,
                 2 => Egoism::Selfless { dc: 12 },
                 _ => Egoism::Selfless { dc: 6 }
            }
        }

        let mut modf = match reproduction.strategy {
            ReproductionStrategy::K { strong: true, .. } => 1,
            ReproductionStrategy::R { strong: true, .. } => -1,
            _ => 0
        };
        modf += match social_organization {
            SocialOrganization::Hive => -1,
            SocialOrganization::Solitary => 1,
            _ => 0
        };

        let f_ego_r = zero_pivot(modf);
        let mut m_ego_r = f_ego_r;
        if matches!(mating, MatingBehavior::Harem) {
            m_ego_r = (m_ego_r + 1).min(3);
        }

        (ego(m_ego_r), ego(f_ego_r))
    }

    pub fn adjust_by_chauempsusp(&mut self, c: Chauvinism, e: Empathy, s: Suspicion) {
        match (*self, c.zp_state(), e.zp_state(), s.zp_state()) {
            (Self::Selfish { dc: 12 }, _, ..=-1, _) |
            (Self::Selfish { dc: 12 }, _, _, 1..)   => *self = Self::Selfish { dc: 9 },
            (Self::Proud, _, ..=-2, _) |
            (Self::Proud, _, _, 2..)   => *self = Self::Selfish { dc: 9 },
            (Self::Proud, _, _, 1..)   => *self = Self::Selfish { dc: 12 },
            (Self::Selfless { dc: 12 }, 2.., _, _) => *self = Self::Selfless { dc: 9 },
            _ => ()
        }
    }
}
