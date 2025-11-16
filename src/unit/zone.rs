use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{celestial::orbital::OrbitEccentricity, unit::{AsMetric, Metric}};

pub enum ZoneDead {
    Z1,
    Z2,
    Both
}

lazy_static! {
    static ref ZONE_FREE_INNER: Metric = 0.au();
    static ref ZONE_FREE_OUTER: Metric = f64::MAX.au();
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

impl From<(&Metric, &Metric)> for Zone {
    fn from(value: (&Metric, &Metric)) -> Self {
        Self::Limited {
            inner: value.0.clone(),
            outer: value.1.clone()
        }
    }
}

impl From<(Metric, Metric)> for Zone {
    fn from(value: (Metric, Metric)) -> Self {
        Self::Limited { inner: value.0, outer: value.1 }
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

    pub fn inner(&self) -> &Metric {
        match self {
            Self::Free => &ZONE_FREE_INNER,
            Self::Limited { inner,..} => inner
        }
    }

    pub fn outer(&self) -> &Metric {
        match self {
            Self::Free => &ZONE_FREE_OUTER,
            Self::Limited { outer,..} => outer
        }
    }
}