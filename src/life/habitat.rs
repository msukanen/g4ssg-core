use dice::{DiceExt, HiLo};

use crate::orbital::OrbitElement;

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
