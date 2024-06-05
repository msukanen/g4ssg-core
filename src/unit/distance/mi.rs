use super::{au::Au, km::Km, ly::Ly, pc::Pc, Distanced};

#[derive(Clone)]
pub struct Mi {
    value: f64,
}

impl Mi {
    pub const KM_IN_MI: f64 = 1.609344;
}

impl Distanced for Mi {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Mi {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Km> for Mi {
    fn from(value: Km) -> Self {
        Self { value: value.raw_value() / Self::KM_IN_MI }
    }
}

impl From<Au> for Mi {
    fn from(value: Au) -> Self {
        Self::from(Km::from(value))
    }
}

impl From<Ly> for Mi {
    fn from(value: Ly) -> Self {
        Self::from(Au::from(value))
    }
}

impl From<Pc> for Mi {
    fn from(value: Pc) -> Self {
        Self::from(Ly::from(value))
    }
}

impl std::fmt::Display for Mi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mi.", self.value)
    }
}
