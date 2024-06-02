use super::{km::Km, ly::Ly, mi::Mi, pc::Pc, Distance};

pub struct Au {
    value: f64,
}

impl Distance for Au {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Au {
    fn from(value: f64) -> Self {
        Au { value }
    }
}

impl From<Km> for Au {
    fn from(value: Km) -> Self {
        Au { value: value.raw_value() / 149_597_871.0 }
    }
}

impl From<Mi> for Au {
    fn from(value: Mi) -> Self {
        Au::from(Km::from(value))
    }
}

impl From<Ly> for Au {
    fn from(value: Ly) -> Self {
        Au { value: value.raw_value() * 63241.0771 }
    }
}

impl From<Pc> for Au {
    fn from(value: Pc) -> Self {
        Au::from(Ly::from(value))
    }
}
