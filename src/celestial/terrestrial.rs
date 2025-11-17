//! Terrestrial Planets, Planetoids, Moons, etc.

use dicebag::DiceExt;

use crate::{celestial::orbital::{orbit_adjacent_to_inner_limit, orbit_adjacent_to_outer_limit}, unit::{Metric, Zone}};
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