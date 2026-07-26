//! Life, as we know it — and otherwise.
use dicebag::hi;
use astrometrics::MetricsInternalType;
use serde::{Deserialize, Serialize};

pub mod bodyplan; use bodyplan::BodyPlan;
pub mod breathing; use breathing::Breathing;
pub mod chemistry; use chemistry::ChemicalBasis;
pub mod growth; use growth::GrowthPattern;
pub mod habitat; use habitat::Habitat;
pub mod locomotion; use locomotion::Locomotion;
pub mod regulation; use regulation::TemperatureRegulation;
pub mod reproduction; use reproduction::*;
pub mod senses;
pub mod size; use size::Size;
pub mod trophics; use trophics::TrophicLevel;

use crate::{celestial::terrestrial::{TerrestrialSubType, climate::Climate}, life::senses::{Senses, special::SpecialSense}};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Life {
    chemical_basis: ChemicalBasis,
    habitat: Habitat,
    trophics: TrophicLevel,
    locomotion: Locomotion,
    size: Size,
    bodyplan: BodyPlan,
    breathing: Option<Breathing>,
    temperature_regulation: TemperatureRegulation,
    growth_pattern: GrowthPattern,
    reproduction: Reproduction,
    senses: Senses,
} impl Life {
    pub fn random(
        sapient: Option<bool>,
        gg: bool,
        sub: Option<TerrestrialSubType>,
        g: Option<MetricsInternalType>,
        hydrocover: Option<f64>,
        climate: Option<Climate>
    ) -> Self {
        let chemical_basis = ChemicalBasis::random_any();
        let habitat = Habitat::random(sub.is_some() && !gg, sub, hydrocover);
        let trophics = TrophicLevel::random(sapient.unwrap_or_else(|| hi!()), habitat, climate.unwrap_or_else(|| Climate::space()));
        let locomotion = Locomotion::random(habitat, trophics, gg);
        let size = Size::random(g, chemical_basis, habitat, trophics, locomotion);
        let bodyplan = BodyPlan::random(gg, g, habitat, trophics, locomotion, size);
        let breathing = Breathing::random(habitat, locomotion);
        let temperature_regulation = TemperatureRegulation::random(habitat, size, breathing);
        let growth_pattern = GrowthPattern::random(habitat, size, locomotion, &bodyplan);
        let reproduction = Reproduction::random(habitat, size, trophics, bodyplan.symmetry, locomotion, temperature_regulation, breathing);
        let senses = Senses::random(g, chemical_basis, size, habitat, trophics, locomotion, &bodyplan, &reproduction);
        //...more to come...
        Self {
            chemical_basis,
            habitat,
            trophics,
            locomotion,
            size,
            bodyplan,
            breathing,
            temperature_regulation,
            growth_pattern,
            reproduction,
            senses,
        }
    }
}
