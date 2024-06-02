use super::{au::Au, km::Km, ly::Ly, pc::Pc, Distance};

pub struct Mi {
    value: f64,
}

impl Mi {
    pub const KM_IN_MI: f64 = 1.609344;
}

impl Distance for Mi {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Mi {
    fn from(value: f64) -> Self {
        Mi { value }
    }
}

impl From<Km> for Mi {
    fn from(value: Km) -> Self {
        Mi { value: value.raw_value() / Self::KM_IN_MI }
    }
}

impl From<Au> for Mi {
    fn from(value: Au) -> Self {
        Mi::from(Km::from(value))
    }
}

impl From<Ly> for Mi {
    fn from(value: Ly) -> Self {
        Mi::from(Au::from(value))
    }
}

impl From<Pc> for Mi {
    fn from(value: Pc) -> Self {
        Mi::from(Ly::from(value))
    }
}
