use super::OrbitalInfo;

pub mod terrestrial;
pub mod climate;
pub mod gasgiant;

pub trait Planet: OrbitalInfo {}

pub enum PlanetType {
    Terrestrial,
    GasGiant,
}
