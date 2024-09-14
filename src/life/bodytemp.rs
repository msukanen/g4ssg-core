use dicebag::DiceExt;

use super::{breathing::Breathing, habitat::{land::LandHabitat, Habitat}, size::SizeCategory};

pub enum TemperatureRegulation {
    Special,
    ColdBlooded(bool),
    PartialRegulation,
    WarmBlooded(bool)
}

impl TemperatureRegulation {
    pub fn random(habitat: &Habitat, size_category: &SizeCategory, breathing: Option<&Breathing>) -> TemperatureRegulation {
        if habitat.is_space() {
            return TemperatureRegulation::Special;
        }

        if let Some(breathing) = breathing {
            let modifier = match breathing {
                Breathing::Gills => -1,
                Breathing::Lungs => 1,
                _ => 0
            } + match size_category {
                SizeCategory::HumanScale |
                SizeCategory::Large      => 1,
                _ => 0
            } + match habitat {
                Habitat::Land(env) => 1 + match env {
                    LandHabitat::Woodlands |
                    LandHabitat::Mountain  => 1,
                    LandHabitat::Arctic    => 2,
                    _ => 0
                },
                _ => 0
            };

            match 2.d6() + modifier {
                ..=4 => TemperatureRegulation::ColdBlooded(true),
                5|6 => TemperatureRegulation::ColdBlooded(false),
                7 => TemperatureRegulation::PartialRegulation,
                8|9 => TemperatureRegulation::WarmBlooded(false),
                10.. => TemperatureRegulation::WarmBlooded(true)
            }
        } else {
            TemperatureRegulation::Special
        }
    }
}
