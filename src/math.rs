mod powerlaw;
use std::f64::consts::PI;

use astrometrics::{AsCelestialRadii, DefoAble, Mass, MetricsInternalType, SpatialUnit};
pub use powerlaw::LogInterpolator;
mod transform;
pub use transform::inverse_transform_sample;

pub const SALPETER_SLOPE_ALPHA: f64 = 2.35;

pub trait Cubert {
    fn cbrt_fast(&self) -> Self;
    fn cbrt(&self) -> Self;
}

#[inline]
const fn cbrt(x: MetricsInternalType, res: usize) -> MetricsInternalType {
    let mut y = x;
    let mut i = 0;
    while i < res {
        y = (2.0 * y + x / (y * y)) / 3.0;
        i += 1;
    }
    y
}

const ONE_PER_3: MetricsInternalType = 1.0/3.0;

impl Cubert for MetricsInternalType {
    #[cfg(feature = "newtonist")]
    #[inline]
    fn cbrt(&self) -> Self { cbrt(*self, 22) }
    #[cfg(not(feature = "newtonist"))]
    #[inline]
    fn cbrt(&self) -> Self { self.powf(ONE_PER_3) }
    #[inline]
    fn cbrt_fast(&self) -> Self { cbrt(*self, 10) }
}

pub fn massdensity_to_radius(mass: Mass, density: f64) -> SpatialUnit {
    (3.0_f64 * mass / (4.0 * PI * density)).raw().cbrt().re()
}
