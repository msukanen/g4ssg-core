mod powerlaw;
pub use powerlaw::LogInterpolator;
mod transform;
pub use transform::inverse_transform_sample;

pub const SALPETER_SLOPE_ALPHA: f64 = 2.35;
