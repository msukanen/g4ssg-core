use self::{arrangement::SexualArrangement, gestation::Gestation, reprstrategy::ReproductionStrategy};

use super::{bodytemp::TemperatureRegulation, breathing::Breathing, habitat::Habitat, locomotion::Locomotion, size::SizeCategory, symmetry::Symmetry, trophiclevel::TrophicLevel};

pub mod arrangement;
pub mod gestation;
pub mod reprstrategy;
pub mod mating;

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

pub trait ArrangementCheck {
    fn is(&self, arrangement: &SexualArrangement) -> bool;
}

pub trait StrategyCheck {
    fn is(&self, strategy: &ReproductionStrategy) -> bool;
}

impl ArrangementCheck for Reproduction {
    fn is(&self, arrangement: &SexualArrangement) -> bool {
        for a in self.arrangement.iter() {
            if a == arrangement {
                return true;
            }
        }
        false
    }
}

impl StrategyCheck for Reproduction {
    fn is(&self, strategy: &ReproductionStrategy) -> bool {
        self.reproduction_strategy == *strategy
    }
}
