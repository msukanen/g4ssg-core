//! Some Metrics
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
use std::ops::{Div, Mul};

use serde::{Deserialize, Serialize};

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

impl Mul<f64> for Metric {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Self::AU(v) => Self::AU(v * rhs),
            Self::SolRadii(v) => Self::SolRadii(v * rhs)
        }
    }
} impl Mul<i32> for Metric {
    type Output = Metric;
    fn mul(self, rhs: i32) -> Self::Output {<Metric as Mul<f64>>::mul(self, rhs as f64)}
}

impl Div<f64> for Metric {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        match self {
            Self::AU(v) => Self::AU(v / rhs),
            Self::SolRadii(v) => Self::SolRadii(v / rhs)
        }
    }
} impl Div<i32> for Metric {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {<Metric as Div<f64>>::div(self, rhs as f64)}
}

impl Mul<&Metric> for f64 {
    type Output = Metric;
    fn mul(self, rhs: &Metric) -> Self::Output {
        match rhs {
            Metric::AU(v) => Metric::AU(v * self),
            Metric::SolRadii(v) => Metric::SolRadii(v * self)
        }
    }
}

impl Mul<Metric> for f64 {
    type Output = Metric;
    fn mul(self, rhs: Metric) -> Self::Output {<f64 as Mul<&Metric>>::mul(self, &rhs)}
}

impl Mul<Metric> for i32 {
    type Output = Metric;
    fn mul(self, rhs: Metric) -> Self::Output {<f64 as Mul<Metric>>::mul(self as f64, rhs)}
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