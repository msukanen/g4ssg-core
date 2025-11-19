//! General Use Math
//! 
//! # `loginterpol`
//! 
//! Power-law interpolator.
/// Logarithmically interpolate a value between `y1` and `y2`
/// based on where `self` sits in relation to `x_min` and `x_max`.
pub trait LogInterpolator {
    fn loginterpol(&self, x_min: f64, y1: f64, x_max: f64, y2: f64) -> f64;
}

impl LogInterpolator for f64 {
    // Albeit the name, this has nothing to do with Interpol logging…
    fn loginterpol(&self, x_min: f64, y1: f64, x_max: f64, y2: f64) -> f64 {
        let lmin = x_min.ln();
        let ly1 = y1.ln();
        let t = (self.ln() - lmin) / (x_max.ln() - lmin);
        (ly1 + t * (y2.ln() - ly1))
            .exp()
    }
}