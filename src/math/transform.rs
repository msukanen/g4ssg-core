use rand::RngExt;

use crate::math::SALPETER_SLOPE_ALPHA;

/// Inverse transform sampling…
/// 
/// # Args
/// `min`– min value, obviously.
/// `max`— ditto.
/// `alpha`— optional; if `None`, default to Salpeter.
pub fn inverse_transform_sample(min: f64, max: f64, alpha: Option<f64>) -> f64 {
    let u = rand::rng().random_range(0.0..1.0);
    // inverse transform sampling
    let pow_exp = 1.0 - if let Some(alpha) = alpha {
        if (alpha - 1.0).abs() < f64::EPSILON {
            // not-a-slope, a.k.a. "a=1" special case… fall back to simple log distribution
            return min * (max / min).powf(u)
        } else {alpha}
    } else {SALPETER_SLOPE_ALPHA};
    let mut a = min.powf(pow_exp);
    let mut b = max.powf(pow_exp);
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    (a + (b - a) * u)
        .powf(1.0 / pow_exp)
        .clamp(min, max)// "Obey the law!", a.k.a. no sneaky border crossers allowed.
}