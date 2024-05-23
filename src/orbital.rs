use self::{asteroidbelt::AsteroidBelt, planet::{gasgiant::GasGiant, terrestrial::Terrestrial}};

pub mod planet;
pub mod asteroidbelt;

pub enum OrbitElement {
    AsteroidBelt(AsteroidBelt),
    Terrestrial(Terrestrial),
    GasGiant(GasGiant),
}

pub trait OrbitalInfo {}
