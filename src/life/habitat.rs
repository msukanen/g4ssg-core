use derivative::Derivative;
use dice::{DiceExt, HiLo};

use crate::orbital::OrbitElement;

pub trait ArcticOrDesert {
    fn is_arctic(&self) -> bool;
    fn is_desert(&self) -> bool;
}

#[derive(Derivative)]
#[derivative(PartialEq)]
pub enum LandHabitat {
    Plains,
    Desert,
    IslandAndBeach,
    Woodlands,
    Swampland,
    Mountain,
    Arctic,
    Jungle
}

impl LandHabitat {
    pub fn random() -> LandHabitat {
        match 3.d6() {
            ..=7 => Self::Plains,
            8 => Self::Desert,
            9 => Self::IslandAndBeach,
            10 => Self::Woodlands,
            11 => Self::Swampland,
            12 => Self::Mountain,
            13 => Self::Arctic,
            14.. => Self::Jungle
        }
    }
}

impl ArcticOrDesert for LandHabitat {
    fn is_arctic(&self) -> bool {
        self == &Self::Arctic
    }

    fn is_desert(&self) -> bool {
        self == &Self::Desert
    }
}

impl std::fmt::Display for LandHabitat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Arctic => "arctic",
            Self::Desert => "desert",
            Self::IslandAndBeach => "island/beach",
            Self::Jungle => "jungle",
            Self::Mountain => "mountains",
            Self::Plains => "plains",
            Self::Swampland => "swampland",
            Self::Woodlands => "woodlands"
        })
    }
}

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

pub enum Habitat {
    Land(LandHabitat),
    Water(WaterHabitat),
    Space,
    Exotica
}

impl std::fmt::Display for Habitat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Land(x) => format!("{x}"),
            Self::Water(x) => format!("{x}"),
            Self::Space => "space".to_string(),
            Self::Exotica => "exotica".to_string()
        })
    }
}

impl ArcticOrDesert for Habitat {
    fn is_arctic(&self) -> bool {
        match self {
            Self::Land(x) => x.is_arctic(),
            Self::Water(x) => x.is_arctic(),
            _ => false
        }
    }

    fn is_desert(&self) -> bool {
        match self {
            Self::Land(x) => x.is_desert(),
            Self::Water(x) => x.is_desert(),
            Self::Space => true,
            _ => false
        }
    }
}


impl From<GenericHabitat> for Habitat {
    fn from(value: GenericHabitat) -> Self {
        match value {
            GenericHabitat::Space => Self::Space,
            GenericHabitat::Exotica => Self::Exotica,
            GenericHabitat::Land => Self::Land(LandHabitat::random()),
            GenericHabitat::Water => Self::Water(WaterHabitat::random())
        }
    }
}

pub enum GenericHabitat {
    Land,
    Water,
    Space,
    Exotica
}

impl From<Option<&OrbitElement>> for GenericHabitat {
    fn from(value: Option<&OrbitElement>) -> Self {
        match value {
            None |
            Some(OrbitElement::AsteroidBelt(_)) => Self::Space,
            Some(OrbitElement::Terrestrial(_)) => if 1.d2().lo() {Self::Land} else {Self::Water},
            Some(OrbitElement::GasGiant(_)) => Self::Water,
            //_ => Self::Exotica // unreachable
        }
    }
}

impl Habitat {
    pub fn random(location: Option<&OrbitElement>) -> Habitat {
        let generic = GenericHabitat::from(location);
        Habitat::from(generic)
    }
}
