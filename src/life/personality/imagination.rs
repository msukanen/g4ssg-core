//! Imagine… imagination!

use serde::{Deserialize, Serialize};

use crate::life::{reproduction::{Reproduction, ReproductionStrategy}, trophics::TrophicLevel};

use super::{ZeroPivotState, zero_pivot, Concentration, Egoism, Empathy};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Imagination {
    Hidebound { iq_penalty: u8 },
    Dull,
    Normal,
    Imaginative { dreamer: bool, v_: u8 },
    Versatile,
    NonstopIdeaFactory,
}

impl ZeroPivotState for Imagination {
    #[inline]
    fn zp_state(&self) -> i8 {
        self.rank().min(6) as i8 - 3
    }
}

impl Imagination {
    const fn rank(&self) -> u8 {
        match self {
            Self::Hidebound { iq_penalty: 0 } => 1,
            Self::Hidebound { .. } => 0,
            Self::Dull => 2,
            Self::Normal => 3,
            Self::Imaginative { dreamer: false,.. } => 4,
            Self::Imaginative { .. } => 5,
            Self::Versatile => 6,
            Self::NonstopIdeaFactory => 7
        }
    }

    pub(super) fn random(trophics: TrophicLevel, reproduction: &Reproduction) -> Self {
        let mut modf = match reproduction.strategy {
            ReproductionStrategy::K { strong: true, .. } => 1,
            ReproductionStrategy::R { strong: true, .. } => -1,
            _ => 0
        };
        if trophics.intersects(TrophicLevel::POUNCING | TrophicLevel::OMNIVORE | TrophicLevel::GATHERING) {
            modf += 1;
        }
        else if trophics.intersects(TrophicLevel::AUTOTROPH | TrophicLevel::FILTER_FEEDER | TrophicLevel::GRAZING) {
            modf -= 1;
        }
        match zero_pivot(modf) {
            ..=-3 => Self::Hidebound { iq_penalty: 1 },
            -2 => Self::Hidebound { iq_penalty: 0 },
            -1 => Self::Dull,
             0 => Self::Normal,
             1 => Self::Imaginative { dreamer: false, v_: 0 },
             2 => Self::Imaginative { dreamer: false, v_: 1 },
             _ => Self::Imaginative { dreamer: false, v_: 2 }
        }
    }

    pub fn adjust_by_coegoemp(&mut self, c: Concentration, e: Egoism, emp: Empathy) {
        match (*self, c.zp_state(), e.zp_state(), emp.zp_state()) {
            (Self::Imaginative { v_: 2,.. }, _, _, ..=0) => *self = Self::NonstopIdeaFactory,
            (Self::Imaginative { v_: 2,.. }
             | Self::Imaginative { v_: 1,.. }, _, 1.., _) |
            (Self::Imaginative { v_: 2,.. }
             | Self::Imaginative { v_: 1,.. }, ..=0, _, _) => *self = Self::Imaginative { dreamer: true, v_: 3 },
            (Self::Imaginative { v_: 0,.. }, 0.., ..=1, _) => *self = Self::Versatile,
            _ => ()
        }
    }
}
