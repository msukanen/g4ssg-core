pub mod ab;
pub mod atmosphere; pub use atmosphere::*;
pub mod axial;
pub mod blackbody;
pub mod gas_giant;  pub use gas_giant::*;
pub mod habitability;
pub mod moons;
pub mod orbital;
pub mod resources;
pub mod rotation;
pub mod size; pub use size::SizeCategory;
pub mod star;
pub mod system_composition;
pub mod terrestrial;

pub trait CountCelestialMajors {
    /// Get number of major celestial objects (stars, black holes, etc.).
    fn num_major_celestials(&self) -> usize;
}
