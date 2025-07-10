//! Planetary basics …
use gasgiant::GasGiant;
use ab::AsteroidBelt;
use terrestrial::Terrestrial;

use crate::core::{designation::IsNamed, orbit::orbital_element::IsOrbitalElement};

pub mod gasgiant;
pub mod ab;
pub mod terrestrial;

pub enum PlanetaryObject {
    AsteroidBelt(AsteroidBelt),
    Terrestrial(Terrestrial),
    GasGiant(GasGiant),
}

pub trait IsPlanet {
    
}

impl IsOrbitalElement for PlanetaryObject {
    
}

impl IsNamed for PlanetaryObject {
    fn designation(&self) -> String {
        match self {
            Self::AsteroidBelt(a) => a.designation(),
            Self::GasGiant(a) => a.designation(),
            Self::Terrestrial(a) => a.designation(),
        }
    }
}
