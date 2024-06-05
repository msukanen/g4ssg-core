use super::{km::Km, ly::Ly, mi::Mi, pc::Pc, Distanced};

#[derive(Clone)]
pub struct Au {
    value: f64,
}

impl Distanced for Au {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Au {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Km> for Au {
    fn from(value: Km) -> Self {
        Self { value: value.raw_value() / 149_597_871.0 }
    }
}

impl From<Mi> for Au {
    fn from(value: Mi) -> Self {
        Self::from(Km::from(value))
    }
}

impl From<Ly> for Au {
    fn from(value: Ly) -> Self {
        Self { value: value.raw_value() * 63241.0771 }
    }
}

impl From<Pc> for Au {
    fn from(value: Pc) -> Self {
        Self::from(Ly::from(value))
    }
}

impl std::fmt::Display for Au {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} AU", self.value)
    }
}
