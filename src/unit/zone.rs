use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

use crate::{celestial::orbital::OrbitEccentricity, unit::{AsMetric, Metric}};

pub enum ZoneDead {
    Z1,
    Z2,
    Both
}

/// A generic "zone"…
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Zone {
    /// A "no limits" zone.
    Free,
    /// A limited zone with inner/outer limits.
    Limited { inner: Metric, outer: Metric }
}

impl From<&OrbitEccentricity> for Zone {
    fn from(ecc: &OrbitEccentricity) -> Self {
        let inner = ecc.min_distance() / 3;
        let outer = ecc.max_distance() * 3;
        Self::Limited { inner, outer }
    }
}

impl Zone {
    pub fn adjust_limits(_z1: &mut Zone, _z2: &mut Zone) -> Result<(), ZoneDead> {
        log::warn!("TODO: adjust_limits() - stub.");
        Ok(())
    }

    pub fn as_range(&self) -> RangeInclusive<Metric> {
        match self {
            Self::Free => 0.au()..=f64::MAX.au(),
            Self::Limited { inner, outer } => inner.clone()..=outer.clone()
        }
    }
}