//! Planetary basics …

use gasgiant::GasGiant;
use ab::AsteroidBelt;
use terrestrial::Terrestrial;

pub mod gasgiant;
pub mod ab;
pub mod terrestrial;

pub enum PlanetaryObject {
    AsteroidBelt(AsteroidBelt),
    Terrestrial(Terrestrial),
    GasGiant(GasGiant),
}
