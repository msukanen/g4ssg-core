use self::{chauvinism::Chauvinism, concentration::Concentration, curiosity::Curiosity, gregariousness::Gregariousness, imagination::Imagination, organization::SocialOrganization};

use super::{senses::Senses, sex::Reproduction, trophiclevel::TrophicLevel};

pub trait PersonalityEffectLevel {
    fn level(&self) -> i32;
}

pub mod organization;
pub mod chauvinism;
pub mod gregariousness;
pub mod imagination;
pub mod concentration;
pub mod curiosity;

pub struct Personality {
    chauvinism: Chauvinism,
    concentration: Concentration,
    curiosity: Curiosity,
    gregariousness: Gregariousness,
    imagination: Imagination,
}

impl Personality {
    pub fn random(
        trophiclevel: &TrophicLevel,
        reproduction: &Reproduction,
        senses: &Senses,
        social_organization: &SocialOrganization,
    ) -> Personality {
        let chauvinism = Chauvinism::random(trophiclevel, social_organization, reproduction);
        let concentration = Concentration::random(trophiclevel, reproduction);
        let curiosity = Curiosity::random(trophiclevel, senses, reproduction);
        let gregariousness = Gregariousness::random(trophiclevel, reproduction, social_organization);
        let imagination = Imagination::random(trophiclevel, reproduction);

        Personality {
            chauvinism,
            concentration,
            curiosity,
            gregariousness,
            imagination,
        }
    }
}
