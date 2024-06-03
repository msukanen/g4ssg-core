use self::{asteroidbelt::AsteroidBelt, planet::{gasgiant::GasGiant, terrestrial::Terrestrial}};

pub mod planet;
pub mod asteroidbelt;
pub mod star;
pub mod separation;
pub mod distance;
pub mod resources;

#[derive(Clone)]
pub enum OrbitElement {
    AsteroidBelt(AsteroidBelt),
    Terrestrial(Terrestrial),
    GasGiant(GasGiant),
}

pub trait OrbitalInfo {
    fn distance(&self) -> f64;
}
