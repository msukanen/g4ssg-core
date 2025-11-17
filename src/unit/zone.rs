use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{celestial::orbital::{ORBIT_RATIO_MAX, OrbitEccentricity}, unit::{AsMetric, Metric}};

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
    /// Adjust the limits…
    //TODO: a stub for now…
    pub fn adjust_limits(_z1: &mut Zone, _z2: &mut Zone) -> Result<(), ZoneDead> {
        log::warn!("TODO: adjust_limits() - stub.");
        Ok(())
    }

    /// Return the zone as an inclusive distance range.
    pub fn as_range(&self) -> RangeInclusive<Metric> {
        match self {
            Self::Free => 0.au()..=f64::MAX.au(),
            Self::Limited { inner, outer } => inner.clone()..=outer.clone()
        }
    }

    /// Get inner edge of the zone.
    pub fn inner(&self) -> &Metric {
        match self {
            Self::Free => &ZONE_FREE_INNER,
            Self::Limited { inner,..} => inner
        }
    }

    /// Get outer edge of the zone.
    pub fn outer(&self) -> &Metric {
        match self {
            Self::Free => &ZONE_FREE_OUTER,
            Self::Limited { outer,..} => outer
        }
    }

    /// See if given `distance` falls between inner and outer limit.
    pub fn contains(&self, distance: &Metric) -> bool {
        match self {
            Self::Free => true,
            Self::Limited { inner, outer } => distance >= inner && distance <= outer
        }
    }

    /// See if any portion of the zone contains `distance` when orbital ratios are applied onto it.
    pub fn orbit_corridor_intersects(&self, distance: &Metric) -> bool {
        let range = self.inner() / ORBIT_RATIO_MAX..=self.outer()*ORBIT_RATIO_MAX;
        range.contains(&distance)
    }
}