use super::{au::Au, km::Km, mi::Mi, pc::Pc, Distanced};

#[derive(Clone)]
pub struct Ly {
    value: f64,
}

impl Distanced for Ly {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Ly {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Au> for Ly {
    fn from(value: Au) -> Self {
        Self { value: value.raw_value() / 63241.0771 }
    }
}

impl From<Km> for Ly {
    fn from(value: Km) -> Self {
        Self::from(Au::from(value))
    }
}

impl From<Mi> for Ly {
    fn from(value: Mi) -> Self {
        Self::from(Au::from(value))
    }
}

impl From<Pc> for Ly {
    fn from(value: Pc) -> Self {
        Self { value: value.raw_value() * 3.26156378 }
    }
}

impl std::fmt::Display for Ly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ly", self.value)
    }
}
