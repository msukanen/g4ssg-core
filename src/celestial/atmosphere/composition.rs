//! Atmospheric composition.

use dicebag::{DiceExt, lo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum MarginalComposition {
    Chlorine,
    Fluorine,
    HiCO2,
    HiO,
    InertGases,
    LowO,
    NitrogenCompounds,
    OrganicToxins,
    Pollutants,
    SulfurCompounds,
} impl MarginalComposition {
    pub fn random() -> Self {
        match 3.d6() {
            ..=4 => if lo!() { Self::Chlorine } else { Self::Fluorine },
            ..=6 => Self::SulfurCompounds,
            7    => Self::NitrogenCompounds,
            ..=9 => Self::OrganicToxins,
            ..=11 => Self::LowO,
            ..=13 => Self::Pollutants,
            14    => Self::HiCO2,
            ..=16 => Self::HiO,
            _     => Self::InertGases
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToxicityLevel {
    Mild,
    High,
    Lethal,
}
