use dice::DiceExt;

use super::ArcticOrDesert;

#[derive(PartialEq)]
pub enum WaterHabitat {
    Banks,
    OpenOcean,
    FreshWaterLakes,
    RiverOrStream,
    TropicalLagoon,
    DeepOceanVents,
    SaltWaterSea,
    Reef
}

impl ArcticOrDesert for WaterHabitat {
    fn is_arctic(&self) -> bool { false }
    fn is_desert(&self) -> bool { false }
}

impl WaterHabitat {
    pub fn random() -> WaterHabitat {
        match 3.d6() {
            ..=7 => Self::Banks,
            8 => Self::OpenOcean,
            9 => Self::FreshWaterLakes,
            10 => Self::RiverOrStream,
            11 => Self::TropicalLagoon,
            12 => Self::DeepOceanVents,
            13 => Self::SaltWaterSea,
            14.. => Self::Reef
        }
    }
}

impl std::fmt::Display for WaterHabitat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Banks => "banks",
            Self::DeepOceanVents => "deep ocean vents",
            Self::FreshWaterLakes => "fresh water lakes",
            Self::OpenOcean => "open ocean",
            Self::Reef => "reef",
            Self::RiverOrStream => "river/stream",
            Self::SaltWaterSea => "salt water sea",
            Self::TropicalLagoon => "tropical lagoon"
        })
    }
}
