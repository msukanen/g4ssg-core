//! Species personality basics.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{intelligence::Intelligence, reproduction::{MatingBehavior, Reproduction}, senses::Senses, size::Size, socialorg::SocialOrganization, trophics::TrophicLevel};

pub mod chauvinism; use chauvinism::Chauvinism;
pub mod concentration; use concentration::Concentration;
pub mod curiosity; use curiosity::Curiosity;
pub mod egoism; use egoism::Egoism;
pub mod empathy; use empathy::Empathy;
pub mod gregariousness; use gregariousness::Gregariousness;
pub mod imagination; use imagination::Imagination;
pub mod playfulness; use playfulness::Playfulness;
pub mod suspicion; use suspicion::Suspicion;

#[inline(always)]
fn zero_pivot(modf: i8) -> i8 {
    zero_pivot_n(1, modf)
}

#[inline]
fn zero_pivot_n(n: i8, modf: i8) -> i8 {
    n.d6() - n.d6() + modf
}

pub trait ZeroPivotState {
    fn zp_state(&self) -> i8;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Personality {
    pub concentration: Concentration,
    pub gregariousness: Gregariousness,
    pub playfulness: Playfulness,
    pub curiosity: Curiosity,
    pub suspicion: Suspicion,
    pub egoism: (Egoism, Egoism),
    pub chauvinism: Chauvinism,
    pub empathy: Empathy,
    pub imagination: Imagination,
} impl Personality {
    pub fn random(
        size: Size,
        trophics: TrophicLevel,
        senses: &Senses,
        reproduction: &Reproduction,
        intelligence: Option<Intelligence>,
        mating: MatingBehavior,
        social_organization: SocialOrganization,
    ) -> Self {
        let concentration = Concentration::random(trophics, reproduction);
        let gregariousness = Gregariousness::random(trophics, reproduction, social_organization);
        let mut playfulness = Playfulness::random(reproduction, intelligence, social_organization);
        // let zp_playfulness = playfulness;
        let curiosity = Curiosity::random(trophics, senses, reproduction, concentration);
        let mut suspicion = Suspicion::random(size, trophics, senses, social_organization);
        let zp_suspicion = suspicion;
        let mut egoism = Egoism::random(reproduction, mating, social_organization);
        let zp_egoism = egoism;
        let mut chauvinism = Chauvinism::random(trophics, reproduction, social_organization);
        let zp_chauvinism = chauvinism;
        let mut empathy = Empathy::random(trophics, reproduction, social_organization);
        let zp_empathy = empathy;
        let mut imagination = Imagination::random(trophics, reproduction);
        // let zp_imagination = imagination;

        // final adjustments…
        chauvinism.adjust_by_empsusp(zp_empathy, zp_suspicion);
        egoism.0.adjust_by_chauempsusp(zp_chauvinism, zp_empathy, zp_suspicion);
        egoism.1.adjust_by_chauempsusp(zp_chauvinism, zp_empathy, zp_suspicion);
        empathy.adjust_by_gregsusp(gregariousness, zp_suspicion);
        imagination.adjust_by_coegoemp(concentration, zp_egoism.1, zp_empathy);
        playfulness.adjust_by_suspicion(zp_suspicion);
        suspicion.adjust_by_chaucurego(zp_chauvinism, curiosity, zp_egoism.1);// we use female ego as that is baseline for Egoism

        Self {
            concentration,
            gregariousness,
            playfulness,
            curiosity,
            suspicion,
            egoism,
            chauvinism,
            empathy,
            imagination,
        }
    }
}
