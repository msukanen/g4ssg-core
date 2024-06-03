#[derive(Clone)]
pub enum AtmosphericMass {
    Trace(f64),
    VeryThin(f64),
    Thin(f64),
    Standard(f64),
    Dense(f64),
    VeryDense(f64),
    Superdense(f64),
}

impl From<f64> for AtmosphericMass {
    fn from(value: f64) -> Self {
        if      value < 0.01 {Self::Trace(value)}
        else if value < 0.51 {Self::VeryThin(value)}
        else if value < 0.81 {Self::Thin(value)}
        else if value < 1.21 {Self::Standard(value)}
        else if value < 1.51 {Self::Dense(value)}
        else if value < 10.0 {Self::VeryDense(value)}
        else                {Self::Superdense(value)}
    }
}

impl AtmosphericMass {
    pub fn raw_value(&self) -> f64 {
        match self {
            Self::Trace(v)     |
            Self::VeryThin(v)  |
            Self::Thin(v)      |
            Self::Standard(v)  |
            Self::Dense(v)     |
            Self::VeryDense(v) |
            Self::Superdense(v)=> *v
        }
    }
}
