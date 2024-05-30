pub mod arrangement;

use crate::starsystem::orbital::{star::limits::OrbitLimits, OrbitElement, OrbitalInfo};

use self::arrangement::GasGiantArrangement;

use super::Planet;

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
    pub fn random(arrangement: GasGiantArrangement, orbit_limits: &OrbitLimits) -> OrbitElement {

        OrbitElement::GasGiant(GasGiant { arrangement })
    }
}
