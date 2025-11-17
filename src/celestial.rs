pub mod ab;
pub mod gas_giant;
//pub use gas_giant::GasGiant;
pub(crate) use gas_giant::{GasGiantArrangement, random_gg_distance};
pub mod orbital;
pub mod star;
pub mod system_composition;
pub mod terrestrial;

pub trait CountCelestialMajors {
    /// Get number of major celestial objects (stars, black holes, etc.).
    fn num_major_celestials(&self) -> usize;
}