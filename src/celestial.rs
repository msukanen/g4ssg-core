pub mod ab;
pub mod atmosphere; pub use atmosphere::*;
pub mod climate;
pub mod gas_giant;  pub use gas_giant::*;
pub mod hydrocover;
pub mod orbital;
pub mod star;
pub mod system_composition;
pub mod terrestrial;

pub trait CountCelestialMajors {
    /// Get number of major celestial objects (stars, black holes, etc.).
    fn num_major_celestials(&self) -> usize;
}
