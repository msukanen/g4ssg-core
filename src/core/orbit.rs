//! Orbital stuff!

use dicebag::{DiceExt, FixedNumberVariance};
pub mod eccentricity;
pub mod separation;
pub mod planet;
pub mod orbital_element;

pub struct OrbitZones {
    inner: Box<dyn IsDistance>,
    outer: Box<dyn IsDistance>,
}

pub trait IsOrbitZone {
    fn inner_limit(&self) -> &dyn IsDistance;
    fn outer_limit(&self) -> &dyn IsDistance;
}

impl IsOrbitZone for OrbitZones {
    fn inner_limit(&self) -> &dyn IsDistance {
        &*self.inner
    }

    fn outer_limit(&self) -> &dyn IsDistance {
        &*self.outer
    }
}

/// Generate a random orbital separation ratio with non-uniform
/// bell-curve like distribution (that favors values around `1.7`).
/// 
/// # Returns
/// A value within (inclusive) range between `1.35` and `2.05`.
//
pub fn gen_orbital_separation_ratio() -> f64 {
    match 3.d6() {
        ..=4 => 1.4,
        ..=6 => 1.5,
        ..=8 => 1.6,
        ..=12 => 1.7,
        ..=14 => 1.8,
        ..=16 => 1.9,
        _ => 2.0
    }.upto_delta(0.05)
}

pub fn gen_orbit_distances(central_pivot_distance: Option<Au>, orbital_zones: &dyn IsOrbitZone) {
    
}
