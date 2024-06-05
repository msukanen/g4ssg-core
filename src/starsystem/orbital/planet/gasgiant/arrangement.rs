use dice::DiceExt;

use crate::{starsystem::orbital::star::limits::orbitlimit::OrbitLimits, unit::distance::{au::Au, Distance}};

/**
 Gas giant arrangement for a star system.
 */
#[derive(PartialEq, Clone, Copy)]
pub enum GasGiantArrangement {
    Conventional(Distance),
    Eccentric(Distance),
    Epistellar(Distance),
}

impl GasGiantArrangement {
    pub fn random(orbit_limits: &OrbitLimits) -> Option<GasGiantArrangement> {
        match 3.d6() {
            ..=10 => None,
            11|12 => {
                let distance = Distance::Au(Au::from(1.0 + ((2.d6() - 2) as f64 * 0.05) + orbit_limits.snowline()));
                if !orbit_limits.is_forbidden_distance(distance) {
                    Some(Self::Conventional(distance))
                } else {None}
            },
            13|14 => {
                let distance = Distance::Au(Au::from(1.d6() as f64 * 0.125 * orbit_limits.snowline()));
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

    pub fn distance(&self) -> Distance {
        match self {
            Self::Conventional(d)|
            Self::Eccentric(d)   |
            Self::Epistellar(d)  => *d
        }
    }
}

impl From<(&GasGiantArrangement, Distance)> for GasGiantArrangement {
    fn from(value: (&GasGiantArrangement, Distance)) -> Self {
        match value.0 {
            GasGiantArrangement::Conventional(_) => GasGiantArrangement::Conventional(value.1),
            GasGiantArrangement::Eccentric(_) => GasGiantArrangement::Eccentric(value.1),
            GasGiantArrangement::Epistellar(_) => GasGiantArrangement::Epistellar(value.1)
        }
    }
}
