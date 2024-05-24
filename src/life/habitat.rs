use derivative::Derivative;
use dice::{DiceExt, HiLo};

use crate::orbital::OrbitElement;

pub trait ArcticOrDesert {
    fn is_arctic(&self) -> bool;
    fn is_desert(&self) -> bool;
}

#[derive(Derivative)]
#[derivative(PartialEq)]
pub(crate) enum LandHabitat {
    Plains,
    Desert,
    IslandAndBeach,
    Woodlands,
    Swampland,
    Mountain,
    Arctic,
    Jungle
}

impl ArcticOrDesert for LandHabitat {
    fn is_arctic(&self) -> bool {
        self == &Self::Arctic
    }

    fn is_desert(&self) -> bool {
        self == &Self::Desert
    }
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

pub(crate) enum WaterHabitat {
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

pub(crate) enum Habitat {
    Land(LandHabitat),
    Water(WaterHabitat),
    Space,
    Exotica
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

pub(crate) enum GenericHabitat {
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

impl GenericHabitat {
    pub fn random(location: Option<&OrbitElement>) -> Habitat {
        let generic = GenericHabitat::from(location);
        Habitat::from(generic)
    }
}
