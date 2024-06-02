use atmosphere::Atmosphere;
use size::Size;

use crate::util::{distance::Distance, mass::Mass};

use super::OrbitalInfo;

pub mod terrestrial;
pub mod climate;
pub mod gasgiant;
pub mod gravity;
pub mod moons;
pub mod size;
pub mod atmosphere;
pub mod hydrographic;
pub mod density;
pub mod core;

pub trait Planet: OrbitalInfo {
    /**
     Get the planet's size.
     */
    fn size(&self) -> Size;
    /**
     Get the planet's major moons.
     */
    fn major_moons(&self) -> &Vec<Size>;
    /**
     Count the planet's moonlets.
     */
    fn moonlets(&self) -> i32;
    /**
     Get the planet's atmosphere, if any.
     */
    fn atmosphere(&self) -> Option<Atmosphere>;
    /**
     Get the planet's gravity, in G.
     */
    fn gravity(&self) -> f64;
    /**
     Get the planet's diameter in km.
     */
    fn diameter(&self) -> Distance;
    /**
     Get the planet's mass (in Earth masses).
     */
    fn mass(&self) -> Mass;
}
