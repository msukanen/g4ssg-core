use crate::unit::distance::{au::Au, ly::Ly, Distance};

/**
 Forbidden zone.
 */
#[derive(Clone, Copy)]
pub struct ForbiddenZone {
    inner: Distance,
    outer: Distance,
}

impl std::fmt::Display for ForbiddenZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FZ {} - {}", self.inner, self.outer)
    }
}

impl From<(Distance, Distance)> for ForbiddenZone {
    fn from(value: (Distance, Distance)) -> Self {
        if value.0 < Distance::Au(Au::from(0.0)) {
            panic!("WTF!")
        }
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
        ForbiddenZone { inner: Distance::Ly(Ly::from(1.0)), outer: Distance::Ly(Ly::from(1.0)) }
    }
}
