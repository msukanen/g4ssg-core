use super::{OrbitElement, OrbitalInfo};

pub struct AsteroidBelt {
    distance: f64,
}

impl OrbitalInfo for AsteroidBelt {
    fn distance(&self) -> f64 {
        self.distance
    }
}

impl AsteroidBelt {
    pub fn random(distance: f64) -> OrbitElement {
        OrbitElement::AsteroidBelt(AsteroidBelt { distance })
    }
}
