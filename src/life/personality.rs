use self::{chauvinism::Chauvinism, concentration::Concentration, curiosity::Curiosity, egoism::Egoism, empathy::Empathy, gregariousness::Gregariousness, imagination::Imagination, organization::SocialOrganization, playfulness::Playfulness, suspicion::Suspicion};

use super::{intelligence::Intelligence, senses::Senses, sex::{mating::MatingBehavior, Reproduction}, size::SizeCategory, trophiclevel::TrophicLevel};

pub trait PersonalityEffectLevel {
    fn level(&self) -> i32;
}

pub mod organization;
pub mod chauvinism;
pub mod gregariousness;
pub mod imagination;
pub mod concentration;
pub mod curiosity;
pub mod egoism;
pub mod empathy;
pub mod suspicion;
pub mod playfulness;

pub struct Personality {
    chauvinism: Chauvinism,
    concentration: Concentration,
    curiosity: Curiosity,
    egoism: Egoism,
    empathy: Empathy,
    gregariousness: Gregariousness,
    imagination: Imagination,
    playfulness: Playfulness,
    suspicion: Suspicion,
}

impl Personality {
    pub fn random(
        size_category: &SizeCategory,
        trophiclevel: &TrophicLevel,
        reproduction: &Reproduction,
        senses: &Senses,
        mating_behavior: &MatingBehavior,
        intelligence: &Intelligence,
        social_organization: &SocialOrganization,
    ) -> Personality {
        let chauvinism = Chauvinism::random(trophiclevel, social_organization, reproduction);
        let concentration = Concentration::random(trophiclevel, reproduction);
        let curiosity = Curiosity::random(trophiclevel, senses, reproduction);
        let empathy = Empathy::random(trophiclevel, social_organization, reproduction);
        let suspicion = Suspicion::random(&curiosity, size_category, trophiclevel, senses.vision(), social_organization);
        let egoism = Egoism::random(&chauvinism, &empathy, &suspicion, mating_behavior, social_organization);
        let gregariousness = Gregariousness::random(trophiclevel, reproduction, social_organization);
        let imagination = Imagination::random(trophiclevel, reproduction);
        let playfulness = Playfulness::random(reproduction, social_organization, intelligence);

        Personality {
            chauvinism,
            concentration,
            curiosity,
            empathy,
            egoism,
            gregariousness,
            imagination,
            playfulness,
            suspicion,
        }
    }
}
