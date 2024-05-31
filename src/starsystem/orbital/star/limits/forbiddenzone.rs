/**
 Forbidden zone.
 */
#[derive(Clone, Copy)]
pub struct ForbiddenZone {
    inner: f64,
    outer: f64,
}

impl From<(f64, f64)> for ForbiddenZone {
    fn from(value: (f64, f64)) -> Self {
        Self {
            inner: value.0 / 3.0,
            outer: value.1 * 3.0
        }
    }
}

impl ForbiddenZone {
    pub fn inner(&self) -> f64 {
        self.inner
    }

    pub fn outer(&self) -> f64 {
        self.outer
    }

    pub fn is_within(&self, distance: f64) -> bool {
        distance >= self.inner() && distance <= self.outer()
    }

    pub fn unlimited() -> ForbiddenZone {
        ForbiddenZone { inner: f64::MAX, outer: f64::MAX }
    }
}
