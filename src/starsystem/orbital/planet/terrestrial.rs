use crate::starsystem::orbital::{OrbitElement, OrbitalInfo};

use super::Planet;

#[derive(Clone, Copy)]
pub struct Terrestrial {
    distance: f64,
}

impl OrbitalInfo for Terrestrial {
    fn distance(&self) -> f64 {
        self.distance
    }
}

impl Planet for Terrestrial {
    
}

impl Terrestrial {
    pub fn random(distance: f64) -> OrbitElement {
        OrbitElement::Terrestrial(Terrestrial { distance })
    }
}
