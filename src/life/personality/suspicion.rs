use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::life::{senses::{Senses, vision::Vision}, size::Size, socialorg::SocialOrganization, trophics::TrophicLevel};

use super::{ZeroPivotState, zero_pivot, Chauvinism, Curiosity, Egoism};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Suspicion {
    Fearfulness { grade: u8, cowardice: bool, paranoia: bool },
    Careful,
    Normal,
    Fearlessness { grade: u8, overconfidence: bool },
    Unfazeable,
}

impl PartialOrd for Suspicion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // paranoia doesn't affect ordering
            (Self::Fearfulness { grade: g1, cowardice: c1, .. },
             Self::Fearfulness { grade: g2, cowardice: c2, .. }) => {
                let g = g2.partial_cmp(g1);
                let Some(Ordering::Equal) = g else { return g };
                (if *c1 && !*c2 { Ordering::Less }
                else if !*c1 && *c2 { Ordering::Greater }
                else { Ordering::Equal }).into()
             }, 
            (Self::Careful, Self::Careful) |
            (Self::Normal, Self::Normal)   => Ordering::Equal.into(),
            (Self::Careful, Self::Fearfulness { .. }) |
            (Self::Normal, Self::Careful | Self::Fearfulness { .. }) => Ordering::Greater.into(),
            (Self::Fearfulness { .. }, _) | (Self::Normal, _) | (Self::Careful, _) => Ordering::Less.into(),
            (Self::Fearlessness { grade: g1, overconfidence: oc1 },
             Self::Fearlessness { grade: g2, overconfidence: oc2 }) => {
                let g = g1.partial_cmp(g2);
                let Some(Ordering::Equal) = g else { return g };
                if *oc1 && !*oc2 { return Ordering::Greater.into() }
                else if !*oc1 && *oc2 { return Ordering::Less.into() }
                Ordering::Equal.into()
             },
            (Self::Fearlessness { .. }, Self::Unfazeable ) => Ordering::Less.into(),
            (Self::Fearlessness { .. }, _ ) |
            (Self::Unfazeable, _)           => Ordering::Greater.into()
        }
    }
}

impl ZeroPivotState for Suspicion {
    #[inline]
    fn zp_state(&self) -> i8 {
        match self {
            Self::Fearfulness { grade: 1, .. } => -2,
            Self::Fearfulness { .. } => -3,
            Self::Careful => -1,
            Self::Normal => 1,
            Self::Fearlessness { grade, .. } => (1 + *grade).min(3) as i8,
            Self::Unfazeable => 3
        }
    }
}

impl Suspicion {
    pub(super) fn random(
        size: Size,
        trophics: TrophicLevel,
        senses: &Senses,
        social_organization: SocialOrganization
    ) -> Self {
        let mut modf =
            if trophics.contains(TrophicLevel::CARNIVORE)
                { -1 }
            else if trophics.contains(TrophicLevel::GRAZING)
                 { 1 }
            else { 0 };
        modf += match senses.vision {
            None | Some(Vision::BadSight { .. }) => 1,
            _ => 0
        };
        modf += match size {
            Size::Small { .. } => 1,
            Size::Large { .. } => -1,
            _ => 0
        };
        modf += match social_organization {
            SocialOrganization::PairBond |
            SocialOrganization::Solitary => 1,
            _ => 0
        };
        match zero_pivot(modf) {
            ..=-3 => Self::Fearfulness { grade: 2, cowardice: trophics.contains(TrophicLevel::HERBIVORE), paranoia: trophics.contains(TrophicLevel::CARNIVORE) },
            -2 => Self::Fearfulness { grade: 1, cowardice: false, paranoia: false },
            -1 => Self::Careful,
             0 => Self::Normal,
             1 => Self::Fearlessness { grade: 1, overconfidence: false },
             2 => Self::Fearlessness { grade: 2, overconfidence: false },
             _ => Self::Fearlessness { grade: 3, overconfidence: false }
        }
    }

    pub fn adjust_by_chaucurego(&mut self, c: Chauvinism, cu: Curiosity, e: Egoism) {
        match (*self, c.zp_state(), cu.zp_state(), e.zp_state()) {
            (Self::Fearfulness { grade: 1, .. }, _, ..=-3, _)
                => *self = Self::Careful,
            (Self::Careful, _, ..=-2, _) => *self = Self::Normal,
            (Self::Fearlessness { grade: 2, ..}, _, _, 2..)
                => *self = Self::Fearlessness { grade: 2, overconfidence: true },
            (Self::Fearlessness { grade: 3, .. }, ..=-3, _, _)
                => *self = Self::Unfazeable,
            (Self::Fearlessness { grade: 3, .. }, _, _, 1..)
                => *self = Self::Fearlessness { grade: 3, overconfidence: true },
            _ => ()
        }
    }
}
