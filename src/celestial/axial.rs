//! Axial tilt.

use dicebag::{DiceExt, InclusiveRandomRange};

use crate::celestial::rotation::TIDAL_BRAKING_TILT_RATIO;

/// Generate random axial tilt (in degrees).
/// 
/// # Args
/// - `tidal_braking`
/// 
pub fn random_axial_tilt(tidal_braking: f64) -> f64 {
    let r2: f64 = (0.0..=10.0).random_of();
    ((match 3.d6() {
        ..=6 => r2,
        ..=9 => 10.0 + r2,
        ..=12 => 20.0 + r2,
        ..=14 => 30.0 + r2,
        ..=16 => 40.0 + r2,
        _ => match 1.d6() {
            ..=2 => 50.0 + r2,
            ..=4 => 60.0 + r2,
            5    => 70.0 + r2,
            _    => 80.0 + r2
        }
    }) - (tidal_braking / TIDAL_BRAKING_TILT_RATIO.unwrap_or_else(|| 1.0)))
    .max(0.0)
}
