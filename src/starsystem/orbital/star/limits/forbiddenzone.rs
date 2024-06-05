use crate::unit::distance::{au::Au, Distance};

/**
 Forbidden zone.
 */
#[derive(Clone, Copy)]
pub struct ForbiddenZone {
    inner: Distance,
    outer: Distance,
}

impl From<(Distance, Distance)> for ForbiddenZone {
    fn from(value: (Distance, Distance)) -> Self {
        Self {
            inner: value.0 / 3.0,
            outer: value.1 * 3.0
        }
    }
}

impl ForbiddenZone {
    pub fn inner(&self) -> Distance {
        self.inner
    }

    pub fn outer(&self) -> Distance {
        self.outer
    }

    pub fn contains(&self, distance: Distance) -> bool {
        distance >= self.inner() && distance <= self.outer()
    }

    pub fn unlimited() -> ForbiddenZone {
        ForbiddenZone { inner: Distance::Au(Au::from(f64::MAX)), outer: Distance::Au(Au::from(f64::MAX)) }
    }
}
