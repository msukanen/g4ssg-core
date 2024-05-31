pub mod arrangement;

use crate::starsystem::orbital::{star::limits::orbitlimit::OrbitLimits, OrbitElement, OrbitalInfo};

use self::arrangement::GasGiantArrangement;

use super::{Planet, Size};

/**
 Gas Giant, obviously.
 */
#[derive(Clone, Copy)]
pub struct GasGiant {
    arrangement: GasGiantArrangement,
    size: Size,
}

impl OrbitalInfo for GasGiant {
    fn distance(&self) -> f64 {
        self.arrangement.distance()
    }
}

impl Planet for GasGiant {
    
}

impl GasGiant {
    pub fn random(inside_snowline_or_first_outside: bool, arrangement: GasGiantArrangement, orbit_limits: &OrbitLimits) -> OrbitElement {
        let size = Size::random_gg(inside_snowline_or_first_outside);
        OrbitElement::GasGiant(GasGiant {
            arrangement,
            size
        })
    }
}
