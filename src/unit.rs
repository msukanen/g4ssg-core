//! Unit types for astromath.
//! 
//! * [`Temperature`] for e.g. Kelvin values and White Dorf markers.
//! * [`Metric`] contains e.g. **au** and **R☉**.
//! * [`Zone`] defines a zone with inner/outer limit (and also `Infinite` variant).
pub mod age;
mod k;
pub use k::Temperature;
mod metrics;
pub use metrics::Metric;
pub use metrics::AsMetric;
pub(crate) use metrics::kroupa_imf_icdf;
mod zone;
pub use zone::Zone;