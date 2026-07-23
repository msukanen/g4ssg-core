//! Life, as we know it — and otherwise.
use dicebag::hi;
use astrometrics::MetricsInternalType;
use serde::{Deserialize, Serialize};

pub mod bodyplan; use bodyplan::BodyPlan;
pub mod breathing; use breathing::Breathing;
pub mod chemistry; use chemistry::ChemicalBasis;
pub mod habitat; use habitat::Habitat;
pub mod locomotion; use locomotion::Locomotion;
pub mod size; use size::Size;
pub mod trophics; use trophics::TrophicLevel;

use crate::{celestial::terrestrial::{TerrestrialSubType, climate::Climate}};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Life {
    chemical_basis: ChemicalBasis,
    habitat: Habitat,
    trophics: TrophicLevel,
    locomotion: Locomotion,
    size: Size,
    bodyplan: BodyPlan,
    breathing: Option<Breathing>,
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
        //...more to come...
        Self {
            chemical_basis,
            habitat,
            trophics,
            locomotion,
            size,
            bodyplan,
            breathing,
        }
    }
}
