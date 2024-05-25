use dice::DiceExt;

use super::ArcticOrDesert;

#[derive(PartialEq)]
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
