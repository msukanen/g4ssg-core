//! # Diameter & Gravity

use astrometrics::{AsCelestialRadii, AsMass, Cubed, DefoAble, Mass, MetricsInternalType, SpatialUnit, Temperature};
use dicebag::{InclusiveRandomRange, PercentageVariance};

use crate::celestial::{SizeCategory, SizeCategory::*};

pub
fn random_radius(size: Option<SizeCategory>, blackbody: Temperature, density: f32) -> SpatialUnit {
    let bk = (blackbody / density).as_f64().sqrt();
    let min = bk * match size {
        Some(Large) => 0.065,
        Some(Medium) => 0.03,
        Some(Small) => 0.024,
        Some(Tiny)  => 0.004,
        _ => 0.0,
    } / 2.0;
    let max = bk * match size {
        Some(Large)  => 0.091,
        Some(Medium) => 0.065,
        Some(Small) => 0.03,
        Some(Tiny) => 0.024,
        _ => 0.0,
    } / 2.0;
    
    if min <= 0.0 || max <= 0.0 { 0.re() }
    else { (min..=max).random_of().re() }
}

pub
fn random_gravity(density: f32, radius: SpatialUnit) -> MetricsInternalType {
    (density * radius.re()).raw().jitter_percentage(5.0)
}

pub
fn random_mass(density: f32, radius: SpatialUnit) -> Mass {
    (density * (2.0_f32 * radius.re()).cubed()).raw().me()
}
