#[derive(Clone)]
pub enum Pressure {
    Trace(f64),
    VeryThin(f64),
    Thin(f64),
    Standard(f64),
    Dense(f64),
    VeryDense(f64),
    Superdense(f64),
}

impl Pressure {
    /**
     Derive atmospheric pressure from atmosphere mass.
     */
    pub fn new(mass: f64) -> Pressure {
        if      mass < 0.01 {Self::Trace(mass)}
        else if mass < 0.51 {Self::VeryThin(mass)}
        else if mass < 0.81 {Self::Thin(mass)}
        else if mass < 1.21 {Self::Standard(mass)}
        else if mass < 1.51 {Self::Dense(mass)}
        else if mass < 10.0 {Self::VeryDense(mass)}
        else                {Self::Superdense(mass)}
    }
}
