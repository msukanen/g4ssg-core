use crate::unit::distance::{au::Au, ly::Ly, Distance};

/**
 Forbidden zone.
 */
#[derive(Clone, Copy)]
pub struct ForbiddenZone {
    /// Inner edge of the zone.
    inner: Distance,
    /// Outer edge of the zone.
    outer: Distance,
}

impl std::fmt::Display for ForbiddenZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FZ {} - {}", self.inner, self.outer)
    }
}

impl From<(Distance, Distance)> for ForbiddenZone {
    /**
     Generate a forbidden zone from a tuple representing inner/outer edges of it.
     */
    fn from(value: (Distance, Distance)) -> Self {
        if value.0 < Distance::Au(Au::from(0.0)) {
            panic!("Something is seriously wrong somewhere - negative distances should NOT happen...")
        }
        
        Self {
            inner: value.0 / 3.0,
            outer: value.1 * 3.0
        }
    }
}

impl ForbiddenZone {
    /**
     Get inner edge of the zone (usually in AU).
     */
    pub fn inner(&self) -> Distance {
        self.inner
    }

    /**
     Get outer edge of the zone (usually in AU).
     */
    pub fn outer(&self) -> Distance {
        self.outer
    }

    /**
     Check if given distance is within the zone bounds.
     */
    pub fn contains(&self, distance: Distance) -> bool {
        distance >= self.inner() && distance <= self.outer()
    }

    /**
     Generate an "unlimited" forbidden zone, a.k.a. unforbidden zone.
     Generally used to represent lack of said zone.
     */
    pub fn none() -> ForbiddenZone {
        ForbiddenZone { inner: Distance::Ly(Ly::from(1.0)), outer: Distance::Ly(Ly::from(1.0)) }
    }
}
