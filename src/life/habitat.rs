pub mod land;
pub mod water;

use dice::{DiceExt, HiLo};

use crate::orbital::OrbitElement;

use self::{land::LandHabitat, water::WaterHabitat};

pub trait ArcticOrDesert {
    fn is_arctic(&self) -> bool;
    fn is_desert(&self) -> bool;
}

#[derive(PartialEq)]
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

enum GenericHabitat {
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

    pub fn is_space(&self) -> bool {
        self == &Habitat::Space
    }

    pub fn is_aquatic(&self) -> bool {
        match self {
            Habitat::Water(_) => true,
            _ => false
        }
    }

    pub fn is(&self, specific: Habitat) -> bool {
        self == &specific
    }
}
