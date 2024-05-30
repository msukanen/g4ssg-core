/**
 Orbit limits.
 */
#[derive(Clone, Copy)]
pub struct OrbitLimits {
    inner: f64,
    outer: f64,
    snowline: f64,
    forbidden_zone: Option<ForbiddenZone>,
}

impl OrbitLimits {
    pub fn new(inner: f64, outer: f64, snowline: f64, forbidden_zone: Option<ForbiddenZone>) -> OrbitLimits {
        OrbitLimits { inner, outer, snowline, forbidden_zone }
    }
}

impl From<(f64, f64, f64, Option<ForbiddenZone>)> for OrbitLimits {
    fn from(value: (f64, f64, f64, Option<ForbiddenZone>)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl OrbitLimits {
    pub fn inner(&self) -> f64 {
        self.inner
    }

    pub fn outer(&self) -> f64 {
        self.outer
    }

    pub fn snowline(&self) -> f64 {
        self.snowline
    }

    pub fn is_forbidden_distance(&self, distance: f64) -> bool {
        match self.forbidden_zone {
            None => false,
            Some(fz) => distance >= fz.inner() && distance <= fz.outer()
        }
    }
}

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
}
