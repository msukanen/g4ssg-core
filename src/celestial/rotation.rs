//! Rotation period etc.

use dicebag::DiceExt;

use crate::celestial::SizeCategory;

const TIDAL_BRAKING_RATIO: f64 = 10.0;
pub(crate) const TIDAL_BRAKING_TILT_RATIO: Option<f64> = Some(200.0);

/// Generate random rotation period (in Earth days).
/// 
/// # Args
/// - `size` category of the planet.
/// - `gg` or not?
/// - `orbital_period`
/// 
pub fn random_rotation_period(size: SizeCategory, gg: bool, tidal_braking: f64, orbital_period: f64) -> f64 {
    use SizeCategory as C;

    if tidal_braking >= 50.0 { return orbital_period; }

    let modf = match (size, gg) {
        (C::Tiny, _) => 18,
        (C::Small, true) => 6,
        (C::Small, _) => 14,
        (_, true) => 0,
        (C::Medium, _) => 10,
        _ => 0
    } + (tidal_braking / TIDAL_BRAKING_RATIO) as i32;

    let r = 3.d6();
    match (r + modf, r) {
        (_, 16) |
        (36..,_) => {
            let r = 1.d6() as f64;
            match 2.d6() {
                ..=6 => return orbital_period,
                7 => r * 2.0,
                8 => r * 5.0,
                9 => r * 10.0,
                10 => r * 20.0,
                11 => r * 50.0,
                _  => r * 100.0
            }
        },
        _ => r as f64 / 24.0
    }
}
