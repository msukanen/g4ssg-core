use crate::unit::distance::Distance;

use super::forbiddenzone::ForbiddenZone;

/**
 Orbit limits.
 */
#[derive(Clone, Copy)]
pub struct OrbitLimits {
    inner: Distance,
    outer: Distance,
    snowline: Distance,
    forbidden_zone: Option<ForbiddenZone>,
}

impl std::fmt::Display for OrbitLimits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OL: {} - {}; {}; {}", self.inner, self.outer, self.snowline, match self.forbidden_zone {
            None => "no-FZ".to_string(),
            Some(z) => format!("{z}")
        })
    }
}

impl OrbitLimits {
    pub fn new(inner: Distance, outer: Distance, snowline: Distance, forbidden_zone: Option<ForbiddenZone>) -> OrbitLimits {
        OrbitLimits { inner, outer, snowline, forbidden_zone }
    }
}

impl From<(Distance, Distance, Distance, Option<ForbiddenZone>)> for OrbitLimits {
    fn from(value: (Distance, Distance, Distance, Option<ForbiddenZone>)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl OrbitLimits {
    /**
     Get inner limit.
     */
    pub fn inner(&self) -> Distance {
        self.inner
    }

    /**
     Get outer limit, optionally clamped by forbidden zone if applicable.
     */
    pub fn outer(&self, forbidden_zone_clamp: bool) -> Distance {
        if let Some(fz) = self.forbidden_zone {
            if forbidden_zone_clamp {
                if self.outer > fz.inner() {
                    fz.inner()
                } else {
                    self.outer
                }
            } else {
                self.outer
            }
        } else {
            self.outer
        }
    }

    pub fn snowline(&self) -> Distance {
        self.snowline
    }

    pub fn is_forbidden_distance(&self, distance: Distance) -> bool {
        match self.forbidden_zone {
            None => false,
            Some(fz) => distance >= fz.inner() && distance <= fz.outer()
        }
    }

    pub fn forbidden_zone(&self) -> ForbiddenZone {
        match self.forbidden_zone {
            None => ForbiddenZone::unlimited(),
            Some(fz) => fz
        }
    }
}
