//! # Planet(oid) Density
//! 

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::celestial::{SizeCategory, terrestrial::TerrestrialSubType};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum Core {
    Icy,
    SmallIron,
    LargeIron,
} impl Core {
    pub fn select(sub: Option<TerrestrialSubType>, size: Option<SizeCategory>) -> Self {
        use TerrestrialSubType as T;
        use SizeCategory as C;

        if sub.is_none() || size.is_none() { return Self::Icy }
        let sub = sub.unwrap();
        let size = size.unwrap();

        match (size, sub) {
            (C::Tiny, T::Ice | T::Sulfur)       |
            (C::Small, T::Hadean | T::Ice)      |
            (C::Medium, T::Hadean | T::Ammonia) |
            (C::Large, T::Ammonia)              => Self::Icy,
            
            (C::Tiny, _) | (C::Small, _) => Self::SmallIron,
            
            _ => Self::LargeIron
        }
    }

    /// Generate random density based on the [Core].
    pub fn random_density(&self) -> f32 {
        (match self {
            Self::Icy => 0.3,
            Self::SmallIron => 0.6,
            Self::LargeIron => 0.8
        }) + 0.1 * match 3.d6() {
            ..=6 => 0,
            ..=10 => 1,
            ..=14 => 2,
            ..=17 => 3,
            _     => 4
        } as f32
    }
}
