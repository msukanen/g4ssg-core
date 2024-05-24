use dice::DiceExt;

use super::{base::{ChemicalBase, ExoticaBase}, habitat::{Habitat, LandHabitat, WaterHabitat}, locomotion::{FlightMode, Locomotion}, trophiclevel::{Herbivore, TrophicLevel}};

pub enum SizeCategory {
    Small,
    HumanScale,
    Large
}

impl SizeCategory {
    pub fn random(base: &ChemicalBase, habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Vec<Locomotion>, local_gravity: f32) -> SizeCategory {
        match 1.d6() + match base {
            ChemicalBase::Exotica(ExoticaBase::Magnetic) => -4,
            _ => 0
        } + match habitat {
            Habitat::Space => 3,
            Habitat::Water(wh) => 1 + match wh {
                WaterHabitat::OpenOcean |
                WaterHabitat::Banks     => 1,
                WaterHabitat::TropicalLagoon |
                WaterHabitat::RiverOrStream => -1,
                _ => 0
            },
            Habitat::Land(lh) => match lh {
                LandHabitat::Plains => 1,
                LandHabitat::IslandAndBeach |
                LandHabitat::Desert         |
                LandHabitat::Mountain       => -1,
                _ => 0
            },
            Habitat::Exotica => 0
        } + match trophiclevel {
            TrophicLevel::Herbivore(Herbivore::Grazing) => 1,
            TrophicLevel::Parasite => -4,
            _ => 0
        }
        + if locomotion.contains(&Locomotion::Slithering) {-1} else {0}
        + if locomotion.contains(&Locomotion::Flight(FlightMode::Winged)) {-3} else {0}
        + if      local_gravity <= 0.4  {2}
          else if local_gravity <= 0.75 {1}
          else if local_gravity <= 1.5  {0}
          else if local_gravity <= 2.0 {-1}
          else {-3}
        {
            ..=2 => Self::Small,
            3|4 => Self::HumanScale,
            _ => Self::Large
        }
    }
}
