use crate::unit::distance::{km::Km, au::Au, ly::Ly, mi::Mi, pc::Pc};

pub mod au;
pub mod km;
pub mod ly;
pub mod mi;
pub mod pc;

pub trait Distanced {
    fn raw_value(&self) -> f64;
}

#[derive(Clone)]
pub enum Distance {
    Km(Km),
    Mi(Mi),
    Au(Au),
    Ly(Ly),
    Pc(Pc),
}

impl Distanced for Distance {
    fn raw_value(&self) -> f64 {
        match self {
            Self::Km(a) => a.raw_value(),
            Self::Mi(a) => a.raw_value(),
            Self::Au(a) => a.raw_value(),
            Self::Ly(a) => a.raw_value(),
            Self::Pc(a) => a.raw_value()
        }
    }
}

impl From<Km> for Distance {
    fn from(value: Km) -> Self {
        Distance::Km(value)
    }
}

impl From<Mi> for Distance {
    fn from(value: Mi) -> Self {
        Distance::Mi(value)
    }
}

impl From<Au> for Distance {
    fn from(value: Au) -> Self {
        Distance::Au(value)
    }
}

impl From<Ly> for Distance {
    fn from(value: Ly) -> Self {
        Distance::Ly(value)
    }
}

impl From<Pc> for Distance {
    fn from(value: Pc) -> Self {
        Distance::Pc(value)
    }
}
