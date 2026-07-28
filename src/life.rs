//! Life, as we know it — and otherwise.
use dicebag::hi;
use astrometrics::MetricsInternalType;
use serde::{Deserialize, Serialize};

pub mod bodyplan; use bodyplan::BodyPlan;
pub mod breathing; use breathing::Breathing;
pub mod chemistry; use chemistry::ChemicalBasis;
pub mod growth; use growth::GrowthPattern;
pub mod habitat; use habitat::Habitat;
pub mod intelligence; use intelligence::Intelligence;
pub mod lifespan; use lifespan::Lifespan;
pub mod locomotion; use locomotion::Locomotion;
pub mod personality; use personality::Personality;
pub mod regulation; use regulation::TemperatureRegulation;
pub mod reproduction; use reproduction::*;
pub mod senses; use senses::Senses;
pub mod size; use size::Size;
pub mod socialorg; use socialorg::SocialOrganization;
pub mod trophics; use trophics::TrophicLevel;

use crate::{celestial::terrestrial::{TerrestrialSubType, climate::Climate}, life::intelligence::Sapience};

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
    intelligence: Option<Intelligence>,
    lifespan: Lifespan,
    mating: MatingBehavior,
    social_organization: SocialOrganization,
    personality: Personality,
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
        let sapient = sapient.unwrap_or_else(|| hi!());
        let trophics = TrophicLevel::random(sapient, habitat, climate.unwrap_or_else(|| Climate::space()));
        let locomotion = Locomotion::random(habitat, trophics, gg);
        let size = Size::random(g, chemical_basis, habitat, trophics, locomotion);
        let bodyplan = BodyPlan::random(gg, g, habitat, trophics, locomotion, size);
        let breathing = Breathing::random(habitat, locomotion);
        let temperature_regulation = TemperatureRegulation::random(habitat, size, breathing);
        let growth_pattern = GrowthPattern::random(habitat, size, locomotion, &bodyplan);
        let reproduction = Reproduction::random(habitat, size, trophics, bodyplan.symmetry, locomotion, temperature_regulation, breathing);
        let senses = Senses::random(g, chemical_basis, size, habitat, trophics, locomotion, &bodyplan, &reproduction);
        let lifespan = Lifespan::derive(sapient, chemical_basis, size, habitat);
        let intelligence = Intelligence::random(sapient, size, trophics, &reproduction, lifespan);
        let mating = MatingBehavior::random(&reproduction);
        let social_organization = SocialOrganization::random(size, trophics, mating);
        let personality = Personality::random(size, trophics, &senses, &reproduction, intelligence, mating, social_organization);
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
            intelligence,
            lifespan,
            mating,
            social_organization,
            personality,
        }
    }
}

impl Sapience for Life {
    fn is_sapient(&self) -> bool {
        match self.intelligence {
            None => false,
            Some(x) => x.is_sapient()
        }
    }
}
