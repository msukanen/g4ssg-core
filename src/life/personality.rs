use self::{chauvinism::Chauvinism, gregariousness::Gregariousness, imagination::Imagination, organization::SocialOrganization};

use super::{sex::Reproduction, trophiclevel::TrophicLevel};

pub trait PersonalityEffectLevel {
    fn level(&self) -> i32;
}

pub mod organization;
pub mod chauvinism;
pub mod gregariousness;
pub mod imagination;

pub struct Personality {
    gregariousness: Gregariousness,
    imagination: Imagination,
}

impl Personality {
    pub fn random(
        trophiclevel: &TrophicLevel,
        reproduction: &Reproduction,
        social_organization: &SocialOrganization,
    ) -> Personality {
        let gregariousness = Gregariousness::random(trophiclevel, reproduction, social_organization);
        let imagination = Imagination::random(trophiclevel, reproduction);

        Personality {
            gregariousness,
            imagination,
        }
    }
}
