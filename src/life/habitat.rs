//! Life habitats.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::celestial::terrestrial::TerrestrialSubType;

/// Land-based life habitat.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum LandHabitat {
    Arctic,
    Desert,
    IslandOrBeach,
    /// Whatever counts as a 'jungle' on the planet…
    Jungle,
    Mountain,
    Plains,
    PlanetaryInterior,
    Swampland,
    /// Whatever happens to go for a 'forest' on the planet…
    Woodlands,
} impl LandHabitat {
    pub fn random() -> Self {
        match 3.d6() {
            ..=7 => Self::Plains,
            8 => Self::Desert,
            9 => Self::IslandOrBeach,
            10 => Self::Woodlands,
            11 => Self::Swampland,
            12 => Self::Mountain,
            13 => Self::Arctic,
            _  => Self::Jungle
        }
    }
}

/// Water (or any fluid) based life habitat.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum WaterHabitat {
    Banks,
    DeepOceanVents,
    /// Submerged (mostly) in a lake of some sort.
    Lake,
    Lagoon,
    OpenOceanSurface,
    Reef,
    RiverOrStream,
    /// Submerged (mostly) in a sea of some sort.
    Sea,
} impl WaterHabitat {
    pub fn random() -> Self {
        match 3.d6() {
            ..=7 => Self::Banks,
            8 => Self::OpenOceanSurface,
            9 => Self::Lake,
            10 => Self::RiverOrStream,
            11 => Self::Lagoon,
            12 => Self::DeepOceanVents,
            13 => Self::Sea,
            _  => Self::Reef
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
    pub struct SpaceHabitat: u8 {
        const INTERSTELLAR   = 1 << 0;
        const DEEP_VOID      = 1 << 1;
        const NEBULAR        = 1 << 2;
        const MAGNETOSPHERIC = 1 << 3;
        const ORBITAL        = 1 << 4;
    }
}

impl SpaceHabitat {
    pub fn random() -> Self {
        let mut flags = SpaceHabitat::empty();

        if 1.d100() <= 40 { flags |= SpaceHabitat::INTERSTELLAR; }
        if 1.d100() <= 25 { flags |= SpaceHabitat::MAGNETOSPHERIC; }
        if 1.d100() <= 15 { flags |= SpaceHabitat::ORBITAL; }
        if flags.contains(Self::ORBITAL) { return flags }
        if 1.d100() <= 10 { flags |= SpaceHabitat::NEBULAR; }
        if 1.d100() <= 10 { flags |= SpaceHabitat::DEEP_VOID; }

        flags
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum Habitat {
    Land (LandHabitat),
    Space (SpaceHabitat),
    Water (WaterHabitat),
} impl Habitat {
    pub fn random(terrestrial: bool, sub: Option<TerrestrialSubType>, hydrocover: Option<f64>, ) -> Self {
        if !terrestrial { return Self::Space (SpaceHabitat::random()) }

        let modf = if sub.is_some() {
            match hydrocover {
                None => return Self::Land (LandHabitat::random()),
                Some(x) if x >= 99.5 => return Self::Water (WaterHabitat::random()),
                Some(x) if x <= 10.0 => -2,
                Some(x) if x <= 50.0 => -1,
                Some(x) if x >= 90.0 => 2,
                Some(x) if x >= 80.0 => 1,
                _ => 0
            }
        } else {0};
        match 1.d6() + modf {
            ..=3 => Self::Land (LandHabitat::random()),
            _    => Self::Water (WaterHabitat::random())
        }
    }
}
