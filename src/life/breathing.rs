//! Most life breathes, in one way or the other.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{habitat::{Habitat, LandHabitat, WaterHabitat}, locomotion::Locomotion};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Breathing {
    Amphibian,
    Gills,
    Lungs { ox_storage: bool }
} impl Breathing {
    pub(crate) fn random(habitat: Habitat, locomotion: Locomotion) -> Option<Self> {
        let mut modf = match &habitat {
            Habitat::Space(_) => return None,
            Habitat::Land(x) => match x {
                LandHabitat::Arctic        |
                LandHabitat::IslandOrBeach |
                LandHabitat::Swampland     => 1,
                _ => 0,
            }
            Habitat::Water(x) => match x {
                WaterHabitat::RiverOrStream |
                WaterHabitat::Lagoon        => 1,
                _ => 0,
            }
        };
        if locomotion.intersects(Locomotion::WINGED_FLIGHT) {
            modf += 2;
        }
        else if locomotion.contains(Locomotion::WALKING) {
            modf += 1;
        }

        match 2.d6() + modf {
            ..=6 => Self::Gills,
            7|8  => Self::Lungs { ox_storage: true },
            9|10 => Self::Amphibian,
            _    => Self::Lungs { ox_storage: false }
        }.into()
    }
}
