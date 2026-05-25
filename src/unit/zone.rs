use std::ops::RangeInclusive;

use astrometrics::{AsSpatialUnit, SpatialUnit};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{celestial::orbital::{ORBIT_RATIO_MAX, OrbitEccentricity}};

pub enum ZoneDead {
    Z1,
    Z2,
    Both
}

lazy_static! {
    static ref ZONE_FREE_INNER: SpatialUnit = 0.au();
    static ref ZONE_FREE_OUTER: SpatialUnit = f64::MAX.au();
}

/// A generic "zone"…
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Zone {
    /// A "no limits" zone.
    Free,
    /// A limited zone with inner/outer limits.
    Limited { inner: SpatialUnit, outer: SpatialUnit }
}

impl From<&OrbitEccentricity> for Zone {
    fn from(ecc: &OrbitEccentricity) -> Self {
        let inner = ecc.min_distance() / 3;
        let outer = ecc.max_distance() * 3;
        Self::Limited { inner, outer }
    }
}

impl From<(&SpatialUnit, &SpatialUnit)> for Zone {
    fn from(value: (&SpatialUnit, &SpatialUnit)) -> Self {
        Self::Limited {
            inner: value.0.clone(),
            outer: value.1.clone()
        }
    }
}

impl From<(SpatialUnit, SpatialUnit)> for Zone {
    fn from(value: (SpatialUnit, SpatialUnit)) -> Self {
        Self::Limited { inner: value.0, outer: value.1 }
    }
}

impl Zone {
    pub const FREE: Self = Self::Free;

    /// Adjust the limits…
    //TODO: a stub for now…
    pub fn adjust_limits(_z1: &mut Zone, _z2: &mut Zone) -> Result<(), ZoneDead> {
        log::warn!("TODO: adjust_limits() - stub.");
        Ok(())
    }

    /// Return the zone as an inclusive distance range.
    pub fn as_range(&self) -> RangeInclusive<SpatialUnit> {
        match self {
            Self::Free => 0.au()..=f64::MAX.au(),
            Self::Limited { inner, outer } => inner.clone()..=outer.clone()
        }
    }

    /// Get inner edge of the zone.
    pub fn inner(&self) -> &SpatialUnit {
        match self {
            Self::Free => &ZONE_FREE_INNER,
            Self::Limited { inner,..} => inner
        }
    }

    /// Get outer edge of the zone.
    pub fn outer(&self) -> &SpatialUnit {
        match self {
            Self::Free => &ZONE_FREE_OUTER,
            Self::Limited { outer,..} => outer
        }
    }

    /// See if given `distance` falls between inner and outer limit.
    pub fn contains(&self, distance: &SpatialUnit) -> bool {
        match self {
            Self::Free => false,
            Self::Limited { inner, outer } => distance >= inner && distance <= outer
        }
    }

    /// See if any portion of the zone contains `distance` (±ORBIT_RATIO_MAX threshold).
    pub fn orbit_corridor_intersects(&self, distance: &SpatialUnit) -> bool {
        if matches!(*self, Self::Free) { return false; }
        let range = self.inner() / ORBIT_RATIO_MAX..=self.outer()*ORBIT_RATIO_MAX;
        range.contains(distance)
    }
}