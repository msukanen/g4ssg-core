//! Chemical basis.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum SiliconBase {
    SulfuricAcid,
    Sulfur,
    Rock,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ExoticBase {
    Hyperstructural,
    NebulaDwelling,
    Machine,
    Magnetic,
    Photonic,
    QuantumMorph,
} impl ExoticBase {
    pub(crate) fn random() -> Self {
        match 3.d6() {
            ..=6 => Self::NebulaDwelling,
            ..=15 => Self::Machine,
            ..=17 => Self::Magnetic,
            _     => Self::Photonic
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ChemicalBasis {
    Hydrogen,
    Ammonia,
    Hydrocarbon,
    Water,
    Chlorine,
    Silicon (SiliconBase),
    Plasma,
    Exotic (ExoticBase),
} impl ChemicalBasis {
    pub fn random_any() -> Self {
        match 3.d6() {
            ..=5 => Self::Hydrogen,
            ..=7 => Self::Ammonia,
            8    => Self::Hydrocarbon,
            ..=11 => Self::Water,
            12 => Self::Chlorine,
            13 => Self::Silicon(SiliconBase::SulfuricAcid),
            14 => Self::Silicon(SiliconBase::Sulfur),
            15 => Self::Silicon(SiliconBase::Rock),
            16 => Self::Plasma,
            _  => Self::Exotic(ExoticBase::random())
        }
    }
}
