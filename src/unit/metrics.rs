//! Some Metrics — Measurement and Otherwise.
//! 
//! * au (astronomical unit)
//! * R☉ (SolRadii)
//! 
//! # `From<f64>`
//! 
//! Generates **au** from given `f64`.
//! 
//! # `Default`
//! 
//! Generates **0 au**.
//! 
//! # Various `Metric::foobarbazxyz()`
//! 
//! * `to_au()` generates an **au** value.
//! * `to_rsun()` generates **R☉** value.
//! 
use core::f64;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::evo::{CFG_STAR_DATA_MAX_MASS, CFG_STAR_DATA_MIN_MASS};

const R_SUN_M: f64 = 6.957e8;
const AU_M: f64 = 1.495978707e11;

/// Various metrics…
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum Metric {
    /// **A**stronomical **U**nit, "**au**".
    AU(f64),
    /// Sol radii, **R☉**.
    SolRadii(f64)
}

impl From<f64> for Metric {
    /// From `f64` to [**au**][Metric::AU].
    fn from(value: f64) -> Self {
        Self::AU(value)
    }
}

impl From<i32> for Metric {
    /// From `i32` to [**au**][Metric::AU].
    fn from(value: i32) -> Self {
        Self::from(value as f64)
    }
}

impl Default for Metric {
    fn default() -> Self {
        Self::AU(0.0)
    }
}

impl Metric {
    /// Convert (if needed) to **au**.
    pub fn to_au(&self) -> Self {
        match self {
            Self::AU(v) => Self::AU(*v),
            Self::SolRadii(v) => Self::AU(*v * (R_SUN_M / AU_M))
        }
    }

    /// Convert (if needed) to **R☉**.
    pub fn to_rsun(&self) -> Self {
        match self {
            Self::SolRadii(v) => Self::SolRadii(*v),
            Self::AU(v) => Self::SolRadii(*v * (AU_M / R_SUN_M))
        }
    }

    /// Get the underlying raw value as `f64`.
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::AU(v)       |
            Self::SolRadii(v) => *v
        }
    }
}

impl Eq for Metric {}
impl Ord for Metric {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::AU(v1), Self::AU(v2))|
            (Self::SolRadii(v1), Self::SolRadii(v2)) => (*v1).partial_cmp(v2).unwrap_or_else(|| panic!("Uh-oh: 'NaN' with '{}'⋚'{}'", *v1, *v2)),
            (Self::AU(_), Self::SolRadii(_)) => self.cmp(&other.to_au()),
            (Self::SolRadii(_), Self::AU(_)) => self.cmp(&other.to_rsun()),
        }
    }
}

//
// Mul
//
impl Mul<f64> for &Metric {
    type Output = Metric;
    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Metric::AU(v) => Metric::AU(v * rhs),
            Metric::SolRadii(v) => Metric::SolRadii(v * rhs)
        }
    }
} impl Mul<i32> for &Metric {
    type Output = Metric;
    fn mul(self, rhs: i32) -> Self::Output {<&Metric as Mul<f64>>::mul(self, rhs as f64)}
} impl Mul<f64> for Metric {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {<&Metric as Mul<f64>>::mul(&self, rhs)}
} impl Mul<i32> for Metric {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {<&Metric as Mul<f64>>::mul(&self, rhs as f64)}
}

//
// Mul
//
impl Mul<&Metric> for f64 {
    type Output = Metric;
    fn mul(self, rhs: &Metric) -> Self::Output {
        match rhs {
            Metric::AU(v) => Metric::AU(v * self),
            Metric::SolRadii(v) => Metric::SolRadii(v * self)
        }
    }
} impl Mul<Metric> for f64 {
    type Output = Metric;
    fn mul(self, rhs: Metric) -> Self::Output {<f64 as Mul<&Metric>>::mul(self, &rhs)}
} impl Mul<Metric> for i32 {
    type Output = Metric;
    fn mul(self, rhs: Metric) -> Self::Output {<f64 as Mul<Metric>>::mul(self as f64, rhs)}
}

//
// Metric × Metric = Metric²
//
impl Mul<&Metric> for &Metric {
    type Output = Metric;
    fn mul(self, rhs: &Metric) -> Self::Output {
        match self {
            Metric::AU(v) => *v * rhs,
            Metric::SolRadii(v) => *v * rhs
        }
    }
} impl Mul for Metric {
    type Output = Metric;
    fn mul(self, rhs: Self) -> Self::Output {<&Metric as Mul<&Metric>>::mul(&self, &rhs)}
} impl Mul<&Metric> for Metric {
    type Output = Metric;
    fn mul(self, rhs: &Metric) -> Self::Output {<&Metric as Mul<&Metric>>::mul(&self, rhs)}
} impl Metric {
    /// Self squared…
    pub fn sq(&self) -> Self {
        self * self
    }
}

//
// Div
//
impl Div<f64> for &Metric {
    type Output = Metric;
    fn div(self, rhs: f64) -> Self::Output {
        match self {
            Metric::AU(v) => Metric::AU(v / rhs),
            Metric::SolRadii(v) => Metric::SolRadii(v / rhs)
        }
    }
} impl Div<i32> for &Metric {
    type Output = Metric;
    fn div(self, rhs: i32) -> Self::Output {<&Metric as Div<f64>>::div(self, rhs as f64)}
} impl Div<f64> for Metric {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {<&Metric as Div<f64>>::div(&self, rhs)}
} impl Div<i32> for Metric {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {<&Metric as Div<f64>>::div(&self, rhs as f64)}
}


//
// Add
//
impl Add<f64> for &Metric {
    type Output = Metric;
    fn add(self, rhs: f64) -> Self::Output {
        match self {
            Metric::AU(v) => Metric::AU(v + rhs),
            Metric::SolRadii(v) => Metric::SolRadii(v + rhs)
        }
    }
} impl Add<f64> for Metric {
    type Output = Metric;
    fn add(self, rhs: f64) -> Self::Output {<&Metric as Add<f64>>::add(&self, rhs)}
}

impl Add<&Metric> for f64 {
    type Output = Metric;
    fn add(self, rhs: &Metric) -> Self::Output {
        match rhs {
            Metric::AU(v) => Metric::AU(*v + self),
            Metric::SolRadii(v) => Metric::SolRadii(*v + self)
        }
    }
} impl Add<Metric> for f64 {
    type Output = Metric;
    fn add(self, rhs: Metric) -> Self::Output {<f64 as Add<&Metric>>::add(self, &rhs)}
}

//
// DivAssign
//
impl DivAssign<f64> for &mut Metric {
    fn div_assign(&mut self, rhs: f64) {
        match self {
            Metric::AU(v) => *v /= rhs,
            Metric::SolRadii(v) => *v /= rhs
        }
    }
} impl DivAssign<f64> for Metric {
    fn div_assign(&mut self, rhs: f64) {
        match self {
            Metric::AU(v) => *v /= rhs,
            Self::SolRadii(v) => *v /= rhs
        }
    }
}

//
// MulAssign
//
impl MulAssign<f64> for &mut Metric {
    fn mul_assign(&mut self, rhs: f64) {
        match self {
            Metric::AU(v) => *v *= rhs,
            Metric::SolRadii(v) => *v *= rhs
        }
    }
} impl MulAssign<f64> for Metric {
    fn mul_assign(&mut self, rhs: f64) {
        match self {
            Metric::AU(v) => *v *= rhs,
            Self::SolRadii(v) => *v *= rhs
        }
    }
}

/// A little convenience extension for making **au** or **R☉** from given value.
pub trait AsMetric {
    fn au(&self) -> Metric;
    fn rsun(&self) -> Metric;
}

impl AsMetric for i32 {
    /// `i32` to [**au**][Metric::AU].
    fn au(&self) -> Metric {
        Metric::AU(*self as f64)
    }

    /// `i32` to [**R☉**][Metric::SolRadii].
    fn rsun(&self) -> Metric {
        Metric::SolRadii(*self as f64)
    }
}

impl AsMetric for f64 {
    /// `f64` to [**au**][Metric::AU].
    fn au(&self) -> Metric {
        Metric::AU(*self)
    }

    /// `f64` to [**R☉**][Metric::SolRadii].
    fn rsun(&self) -> Metric {
        Metric::SolRadii(*self)
    }
}

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
