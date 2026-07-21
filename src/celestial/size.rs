use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i8)]
pub enum SizeCategory {
    /// Not really a planet at all; a literal *cluster* of asteroids more or less tightly packed.
    AsteroidCluster = -1,
    /// A tiny planet/planetoid. In some contexts synonymous with 'Moon'.
    Tiny = 0,
    /// E.g. Mercury (for terrestrials)
    Small,
    /// E.g. Earth, Venus (for terrestrials); Uranus
    Medium,
    /// E.g. Super-Earths | Jupiter.
    Large,
}

impl SizeCategory {
    pub fn prev(&self) -> Self {
        match self {
            Self::AsteroidCluster |
            Self::Tiny => Self::AsteroidCluster,
            Self::Small => Self::Tiny,
            Self::Medium => Self::Small,
            Self::Large => Self::Medium
        }
    }

    pub fn random_stepdown(&self) -> Self {
        let mut prev = *self;
        for _ in 0..(match 3.d6() { ..=11 => 3, ..=14 => 2, _ => 1}) {
            prev = prev.prev()
        }
        prev
    }
}
