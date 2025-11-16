pub mod orbital;
pub mod star;
pub mod system_composition;

pub trait CountCelestialMajors {
    /// Get number of major celestial objects (stars, black holes, etc.).
    fn num_major_celestials(&self) -> usize;
}