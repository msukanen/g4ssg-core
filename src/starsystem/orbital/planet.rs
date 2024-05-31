use size::Size;

use super::OrbitalInfo;

pub mod terrestrial;
pub mod climate;
pub mod gasgiant;
pub mod gravity;
pub mod moons;
pub mod size;

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
}

pub enum PlanetType {
    Terrestrial,
    GasGiant,
}
