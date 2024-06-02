use super::{au::Au, ly::Ly, mi::Mi, pc::Pc, Distanced};

#[derive(Clone)]
pub struct Km {
    value: f64,
}

impl Distanced for Km {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Km {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Mi> for Km {
    fn from(value: Mi) -> Self {
        Self { value: value.raw_value() * Mi::KM_IN_MI }
    }
}

impl From<Au> for Km {
    fn from(value: Au) -> Self {
        Self { value: value.raw_value() * 149_597_871.0 }
    }
}

impl From<Ly> for Km {
    fn from(value: Ly) -> Self {
        Self::from(Au::from(value))
    }
}

impl From<Pc> for Km {
    fn from(value: Pc) -> Self {
        Self::from(Au::from(value))
    }
}
