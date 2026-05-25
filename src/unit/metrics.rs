//! Some Metrics
use core::f64;

use lazy_static::lazy_static;
use rand::RngExt;

use crate::evo::{CFG_STAR_DATA_MAX_MASS, CFG_STAR_DATA_MIN_MASS};

const R_SUN_M: f64 = 6.957e8;
const AU_M: f64 = 1.495978707e11;

// Kroupa IMF parameters
const ALPHA_0: f64 = 0.3;  // m < 0.08 M☉
const ALPHA_1: f64 = 1.3;  // 0.08 M☉ <= m < 0.5 M☉
const ALPHA_2: f64 = 2.3;  // m >= 0.5 M☉
fn calc_segment_area(min: f64, max: f64, alpha: f64) -> f64 {
    let eff_min = min.max(*CFG_STAR_DATA_MIN_MASS);
    let eff_max = max.min(*CFG_STAR_DATA_MAX_MASS);
    if eff_max <= eff_min {
        return 0.0;
    }
    let p = 1.0 - alpha;
    (eff_max.powf(p) / p) - (eff_min.powf(p) / p)
}
lazy_static! {
    // Integrated probability densities for each segment...
    static ref KROUPA_C1: f64 = calc_segment_area(*CFG_STAR_DATA_MIN_MASS, 0.08, ALPHA_0);
    static ref KROUPA_C2: f64 = calc_segment_area(0.08, 0.5, ALPHA_1);
    static ref KROUPA_C3: f64 = calc_segment_area(0.5, *CFG_STAR_DATA_MAX_MASS, ALPHA_2);
    // Precalc normalization etc.
    static ref KROUPA_TOTAL_AREA: f64 = *KROUPA_C1 + *KROUPA_C2 + *KROUPA_C3;
    static ref KROUPA_A1: f64 = *KROUPA_C1 / *KROUPA_TOTAL_AREA;
    static ref KROUPA_A2: f64 = (*KROUPA_C1 + *KROUPA_C2) / *KROUPA_TOTAL_AREA;
}

/// Generate a random M☉ value between ~0.1M☉ and ~100M☉.
/// 
/// We try to follow Kroupa's broken power-law distribution function for this, with coefficients somewhat simplified.
/// 
/// # Returns
/// `0.1..100.0`
/// 
/// # Example
/// ```
/// let stellar_mass = kroupa_imf_icdf();
/// ```
// The PDF is `dN/dm ~ m^-alpha`.
pub(crate) fn kroupa_imf_icdf() -> f64 {
    let u: f64 = rand::rng().random();
    let derive_mass =
        |low: f64, hi: f64, u: f64, alpha: f64| {
        let p = 1.0 - alpha;
        ((hi.powf(p) - low.powf(p)) * u + low.powf(p)).powf(1.0/p)
    };
    
    let (low, hi, u, alpha) = if u < *KROUPA_A1 {
        (CFG_STAR_DATA_MIN_MASS.max(0.0),
         0.08_f64.min(*CFG_STAR_DATA_MAX_MASS),
         u, ALPHA_0)
    } else if u < *KROUPA_A2 {
        // Sample from the 0.1-0.5 M☉ range
        (0.08_f64.max(*CFG_STAR_DATA_MIN_MASS),
         0.5_f64.min(*CFG_STAR_DATA_MAX_MASS),
         (u - *KROUPA_A1) / (*KROUPA_A2 - *KROUPA_A1),
         ALPHA_1)
    } else {
        (0.5_f64.max(*CFG_STAR_DATA_MIN_MASS),
         CFG_STAR_DATA_MAX_MASS.min(100.0),
         (u - *KROUPA_A2) / (1.0 - *KROUPA_A2),
         ALPHA_2)
    };
    derive_mass(low, hi, u, alpha)//.max(0.0) //uncomment .max() if not trusting the equation to auto-clamp at 0.0 ...
}
