//! Terrestrial Planets, Planetoids, Moons, etc.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum SizeCategory {
    /// A tiny planet/planetoid. In some contexts synonymous with 'Moon'.
    Tiny,
    /// Mercury, etc.
    Small,
    /// Earth, Venus, etc.
    Medium,
    /// E.g. Super-Earths, etc.
    Large,
}