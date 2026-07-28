//! Intelligence, schmintelligence.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{lifespan::Lifespan, reproduction::{Reproduction, ReproductionStrategy}, size::Size, trophics::TrophicLevel};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Intelligence {
    Preprogrammed,
    Low { iq: u8 },
    High { iq: u8 },
    Presapient { iq: u8 },
    Sapient { iq: u8 },
} impl Intelligence {
    /// Return random [Intelligence] (or lack of such thereof).
    pub fn random(sapient: bool, size: Size, trophics: TrophicLevel, reproduction: &Reproduction, lifespan: Lifespan) -> Option<Self> {
        let mut modf: i32 =
            if trophics.intersects(TrophicLevel::AUTOTROPH | TrophicLevel::FILTER_FEEDER | TrophicLevel::GRAZING)
                { -1 }
            else if trophics.intersects(TrophicLevel::CARNIVORE | TrophicLevel::OMNIVORE)
                 { 1 }
            else { 0 };
        if matches!(size, Size::Small { .. }) {
            modf -= 1;
        }
        modf += match reproduction.strategy {
            ReproductionStrategy::K { strong: true, .. } => 1,
            ReproductionStrategy::R { strong: true, .. } => -1,
            _ => 0
        };

        if sapient {
            // min-IQ for sapient: 6
            Self::Sapient { iq: (1.d6() + 5 + modf).max(6) as u8 }.into()
        } else {
            modf += match lifespan {
                Lifespan::Normal          |
                Lifespan::Extended { .. } |
                Lifespan::Immortal        => 1,
                _ => 0
            };
            match 2.d6() + modf {
                ..=3 => return None,
                4|5  => Self::Preprogrammed,
                ..=8 => Self::Low { iq: 1.d3() },
                9|10 => Self::High { iq: 1.d3() + 2 },
                _    => Self::Presapient { iq: 1.d6().max(5) }
            }.into()
        }
    }
}

pub trait Sapience {
    fn is_sapient(&self) -> bool;
}

impl Sapience for Intelligence {
    #[inline]
    fn is_sapient(&self) -> bool {
        matches!(self, Self::Sapient { .. })
    }
}
