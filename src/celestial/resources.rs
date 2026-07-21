//! Resources.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::celestial::SizeCategory;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceValue {
    Worthless,
    VeryScant,
    Scant,
    VeryPoor,
    Poor,
    Average,
    Abundant,
    VeryAbundant,
    Rich,
    VeryRich,
    Motherlode,
} impl ResourceValue {
    /// Generate random resource value of given `size` object.
    pub fn random(size: Option<SizeCategory>) -> Self {
        use ResourceValue::*;
        match size {
            None | Some(SizeCategory::AsteroidCluster) => match 3.d6() {
                ..=3 => Worthless,
                4    => VeryScant,
                5    => Scant,
                ..=7 => VeryPoor,
                ..=9 => Poor,
                ..=11 => Average,
                ..=13 => Abundant,
                ..=15 => VeryAbundant,
                16    => Rich,
                17    => VeryRich,
                _     => Motherlode,
            },

            Some(sz) => match 3.d6() + sz as i8 {
                ..=2 => Self::Scant,
                ..=4 => Self::VeryPoor,
                ..=7 => Self::Poor,
                ..=13 => Self::Average,
                ..=16 => Self::Abundant,
                ..=18 => Self::VeryAbundant,
                _     => Self::Rich
            }
        }
    }

    /// Get resource's value modifier, *rvm*.
    #[inline(always)]
    pub const fn rvm(&self) -> i32 {
        match self {
            Self::Worthless => -5,
            Self::VeryScant => -4,
            Self::Scant     => -3,
            Self::VeryPoor  => -2,
            Self::Poor      => -1,
            Self::Average   =>  0,
            Self::Abundant  =>  1,
            Self::VeryAbundant => 2,
            Self::Rich      =>  3,
            Self::VeryRich  =>  4,
            Self::Motherlode => 5,
        }
    }
}
