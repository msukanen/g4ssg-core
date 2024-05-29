use crate::{advantages::{self, Advantage, AdvantageContainer}, disadvantages::{self, overconfidence::Overconfidence, Disadvantage, DisadvantageContainer}};

use self::{chauvinism::Chauvinism, concentration::Concentration, curiosity::Curiosity, egoism::Egoism, empathy::Empathy, gregariousness::Gregariousness, imagination::Imagination, organization::SocialOrganization, playfulness::Playfulness, suspicion::Suspicion};

use super::{intelligence::Intelligence, senses::Senses, sex::{mating::MatingBehavior, Reproduction}, size::SizeCategory, trophiclevel::TrophicLevel};

pub trait PersonalityEffectLevel {
    fn level(&self) -> i32;
}

pub trait PersonalityEffect {
    fn gain(&self, personality: &Personality, trophiclevel: &TrophicLevel) -> (Vec<Box<dyn Disadvantage>>, Vec<Box<dyn Advantage>>) {
        let _ = personality;
        let _ = trophiclevel;
        (vec![], vec![])
    }
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
    overconfident: bool,

    disadvantages: Vec<Box<dyn Disadvantage>>,
    advantages: Vec<Box<dyn Advantage>>,
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
        let mut curiosity = Curiosity::random(trophiclevel, senses, reproduction);
        let mut empathy = Empathy::random(trophiclevel, social_organization, reproduction);
        let mut suspicion = Suspicion::random(&curiosity, size_category, trophiclevel, senses.vision(), social_organization);
        let mut egoism = Egoism::random(&chauvinism, &empathy, &suspicion, mating_behavior, social_organization);
        let gregariousness = Gregariousness::random(trophiclevel, reproduction, social_organization);
        let imagination = Imagination::random(trophiclevel, reproduction);
        let playfulness = Playfulness::random(reproduction, social_organization, intelligence);
        // Shift the values if needed...
        curiosity = curiosity.shift_based_on(&suspicion, &concentration);
        empathy = empathy.shift_based_on(&gregariousness, &suspicion);
        egoism = egoism.shift_based_on(&suspicion, &chauvinism, &empathy);
        suspicion = suspicion.shift_based_on(&curiosity);

        let mut personality = Personality {
            chauvinism,
            concentration,
            curiosity,
            empathy,
            egoism,
            gregariousness,
            imagination,
            playfulness,
            suspicion,
            overconfident: false,
            disadvantages: vec![],
            advantages: vec![],
        };

        let (d,a) = personality.chauvinism.gain(&personality, trophiclevel);
        personality.disadvantages.extend(d);
        personality.advantages.extend(a);
        let (d,a) = personality.concentration.gain(&personality, trophiclevel);
        personality.disadvantages.extend(d);
        personality.advantages.extend(a);
        let (d,a) = personality.curiosity.gain(&personality, trophiclevel);
        personality.disadvantages.extend(d);
        personality.advantages.extend(a);
        let (d,a) = personality.empathy.gain(&personality, trophiclevel);
        personality.disadvantages.extend(d);
        personality.advantages.extend(a);
        let (d,a) = personality.gregariousness.gain(&personality, trophiclevel);
        personality.disadvantages.extend(d);
        personality.advantages.extend(a);
        let (d,a) = personality.imagination.gain(&personality, trophiclevel);
        personality.disadvantages.extend(d);
        personality.advantages.extend(a);
        let (d,a) = personality.suspicion.gain(&personality, trophiclevel);
        personality.disadvantages.extend(d);
        personality.advantages.extend(a);

        for d in personality.disadvantages.iter() {
            if d.is_overconfidence() {
                personality.overconfident = true;
                break;
            }
        }

        let (d,a) = personality.playfulness.gain(&personality, trophiclevel);
        personality.disadvantages.extend(d);
        personality.advantages.extend(a);

        personality
    }
}

impl DisadvantageContainer for Personality {
    fn disadvantages(&self) -> &Vec<Box<dyn Disadvantage>> {
        &self.disadvantages
    }
}

impl AdvantageContainer for Personality {
    fn advantages(&self) -> &Vec<Box<dyn Advantage>> {
        &self.advantages
    }
}
