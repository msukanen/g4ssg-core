use dice::DiceExt;

use crate::starsystem::orbital::{star::limits::OrbitLimits, OrbitElement, OrbitalInfo};

use super::Planet;

/**
 Gas giant arrangement for a star system.
 */
#[derive(PartialEq, Clone, Copy)]
pub enum GasGiantArrangement {
    Conventional(f64),
    Eccentric(f64),
    Epistellar(f64),
}

impl GasGiantArrangement {
    pub fn random(orbit_limits: &OrbitLimits) -> Option<GasGiantArrangement> {
        match 3.d6() {
            ..=10 => None,
            11|12 => {
                let distance = 1.0 + ((2.d6() - 2) as f64 * 0.05) + orbit_limits.snowline();
                if !orbit_limits.is_forbidden_distance(distance) {
                    Some(Self::Conventional(distance))
                } else {None}
            },
            13|14 => {
                let distance = 1.d6() as f64 * 0.125 * orbit_limits.snowline();
                if !orbit_limits.is_forbidden_distance(distance) {
                    Some(Self::Eccentric(distance))
                } else {None}
            },
            15.. => {
                let distance = 3.d6() as f64 / 10.0 * orbit_limits.inner();
                if !orbit_limits.is_forbidden_distance(distance) {
                    Some(Self::Epistellar(distance))
                } else {None}
            }
        }
    }

    pub fn distance(&self) -> f64 {
        match self {
            Self::Conventional(d)|
            Self::Eccentric(d)   |
            Self::Epistellar(d)  => *d
        }
    }
}

/**
 Gas Giant, obviously.
 */
pub struct GasGiant {
    arrangement: GasGiantArrangement,
}

impl OrbitalInfo for GasGiant {
    fn distance(&self) -> f64 {
        self.arrangement.distance()
    }
}

impl Planet for GasGiant {
    
}

impl GasGiant {
    pub fn random(arrangement: GasGiantArrangement) -> OrbitElement {
        OrbitElement::GasGiant(GasGiant { arrangement })
    }
}
