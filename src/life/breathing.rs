use dice::DiceExt;

use super::{habitat::{land::LandHabitat, water::WaterHabitat, Habitat}, locomotion::{Locomotion, LocomotionMode}};

pub enum Breathing {
    Lungs,
    Gills,
    LungsWithOxygenStorage,
    GillsAndLungs,
}

impl Breathing {
    pub fn random(habitat: &Habitat, locomotion: &Locomotion) -> Option<Breathing> {
        match habitat {
            Habitat::Space   |
            Habitat::Exotica => None,
            Habitat::Water(WaterHabitat::DeepOceanVents) => Some(Breathing::Gills),
            _ => {
                let modifier = match habitat {
                    Habitat::Land(LandHabitat::Arctic)         |
                    Habitat::Land(LandHabitat::Swampland)      |
                    Habitat::Land(LandHabitat::IslandAndBeach) |
                    Habitat::Water(WaterHabitat::RiverOrStream)|
                    Habitat::Water(WaterHabitat::TropicalLagoon)=> 1,
                    _ => 0
                } + if locomotion.primary_match(LocomotionMode::Walking) ||
                       locomotion.secondary_match(LocomotionMode::Walking) {1}
                    else if locomotion.is_flyer() ||
                            locomotion.primary_match(LocomotionMode::Digging) ||
                            locomotion.secondary_match(LocomotionMode::Digging) {2}
                    else {0};
                match 2.d6() + modifier {
                    ..=6 => Some(Breathing::Gills),
                    7|8 => Some(Breathing::LungsWithOxygenStorage),
                    9|10 => Some(Breathing::GillsAndLungs),
                    11.. => Some(Self::Lungs)
                }
            }
        }
    }
}
