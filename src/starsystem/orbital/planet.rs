use super::OrbitalInfo;

pub mod terrestrial;
pub mod climate;
pub mod gasgiant;
pub mod gravity;
pub mod moons;
pub mod size;

pub trait Planet: OrbitalInfo {}

pub enum PlanetType {
    Terrestrial,
    GasGiant,
}
