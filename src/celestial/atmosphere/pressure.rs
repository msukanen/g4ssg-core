//! Atmospheric pressure — high stress or otherwise.
//! 
//! Atmospheric pressure is measured in *atmospheres* (**atm**), with
//! **1 atm** being equal to the avg. sealevel air pressure on Earth.
//! 
use astrometrics::{DefoAble, Mass, MetricsInternalType, SpatialUnit, Temperature};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtmCategory {
    Thin,
    Standard,
    Dense,
    Superdense,
    Degenerate,
}

pub enum AtmState {
    Rarefied (MetricsInternalType),
    Ideal (MetricsInternalType),
    Supercritical (MetricsInternalType),
    MetallicCore (MetricsInternalType),
}
