use msuk_scifi::unit::distance::Distance;

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
    fn distance(&self) -> Distance;
}

impl std::fmt::Display for OrbitElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::AsteroidBelt(a) => format!("{a}"),
            Self::GasGiant(a) => format!("{a}"),
            Self::Terrestrial(a) => format!("{a}")
        })
    }
}
