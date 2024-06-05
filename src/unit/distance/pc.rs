use super::{au::Au, km::Km, ly::Ly, mi::Mi, Distanced};

#[derive(Clone)]
pub struct Pc {
    value: f64,
}

impl Distanced for Pc {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Pc {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Ly> for Pc {
    fn from(value: Ly) -> Self {
        Self { value: value.raw_value() /  3.26156378 }
    }
}

impl From<Au> for Pc {
    fn from(value: Au) -> Self {
        Self::from(Ly::from(value))
    }
}

impl From<Mi> for Pc {
    fn from(value: Mi) -> Self {
        Self::from(Ly::from(value))
    }
}

impl From<Km> for Pc {
    fn from(value: Km) -> Self {
        Self::from(Ly::from(value))
    }
}

impl std::fmt::Display for Pc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} pc", self.value)
    }
}
