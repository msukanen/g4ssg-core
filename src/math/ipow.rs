//! General Use Math
//! 
//! # `ipow`
//! 
//! Power-law interpolator.
/// Logarithmically interpolate a value between `y1` and `y2`
/// based on where `x` sits in relation to `x_min` and `x_max`.
pub fn ipow(x: f64, x_min: f64, y1: f64, x_max: f64, y2: f64) -> f64 {
    let lx = x.ln();
    let lmin = x_min.ln();
    let lmax = x_max.ln();
    let ly1 = y1.ln();
    let ly2 = y2.ln();
    let t = (lx - lmin) / (lmax - lmin);
    (ly1 + t * (ly2 - ly1))
        .exp()
}
