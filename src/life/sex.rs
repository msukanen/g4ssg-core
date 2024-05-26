use self::{arrangement::SexualArrangement, gestation::Gestation, reprstrategy::ReproductionStrategy};

use super::{bodytemp::TemperatureRegulation, breathing::Breathing, habitat::Habitat, locomotion::Locomotion, size::SizeCategory, symmetry::Symmetry, trophiclevel::TrophicLevel};

pub mod arrangement;
pub mod gestation;
pub mod reprstrategy;

pub struct Reproduction {
    arrangement: Vec<SexualArrangement>,
    gestation: Gestation,
    reproduction_strategy: ReproductionStrategy,
}

impl Reproduction {
    pub fn random(
        habitat: &Habitat,
        symmetry: &Symmetry,
        trophiclevel: &TrophicLevel,
        locomotion: &Locomotion,
        size_category: &SizeCategory,
        breathing: Option<&Breathing>,
        temperature_regulation: &TemperatureRegulation,
    ) -> Reproduction {
        let arrangement = SexualArrangement::random(symmetry, trophiclevel, locomotion);
        let gestation = Gestation::random(habitat, locomotion, breathing, temperature_regulation);
        let reproduction_strategy = ReproductionStrategy::random(size_category, &gestation);

        Reproduction {
            arrangement,
            gestation,
            reproduction_strategy,
        }
    }

    pub fn arrangement(&self) -> &Vec<SexualArrangement> {
        &self.arrangement
    }

    pub fn gestation(&self) -> &Gestation {
        &self.gestation
    }

    pub fn strategy(&self) -> &ReproductionStrategy {
        &self.reproduction_strategy
    }

    pub fn is_sexual_reproduction(&self) -> bool {
        for arrangement in self.arrangement() {
            if arrangement.is_sexual_reproduction() {
                return true;
            }
        }
        false
    }
}
