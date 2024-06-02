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
        Km { value }
    }
}

impl From<Mi> for Km {
    fn from(value: Mi) -> Self {
        Km { value: value.raw_value() * Mi::KM_IN_MI }
    }
}

impl From<Au> for Km {
    fn from(value: Au) -> Self {
        Km { value: value.raw_value() * 149_597_871.0 }
    }
}

impl From<Ly> for Km {
    fn from(value: Ly) -> Self {
        Km::from(Au::from(value))
    }
}

impl From<Pc> for Km {
    fn from(value: Pc) -> Self {
        Km::from(Au::from(value))
    }
}
