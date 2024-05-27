use crate::orbital::OrbitElement;

use self::{appendages::{manipulators::Manipulator, numlimbs::NumberOfLimbs}, base::LifeBase, bodytemp::TemperatureRegulation, breathing::Breathing, growthpattern::GrowthPattern, habitat::Habitat, intelligence::Intelligence, lifespan::Lifespan, locomotion::Locomotion, organization::SocialOrganization, senses::Senses, sex::{mating::MatingBehavior, Reproduction}, size::{Size, SizeCategory}, skeleton::Skeleton, symmetry::Symmetry, tail::Tail, trophiclevel::TrophicLevel};

pub mod base;
pub mod habitat;
pub mod trophiclevel;
pub mod locomotion;
pub mod size;
pub mod symmetry;
pub mod tail;
pub mod skeleton;
pub mod skin;
pub mod breathing;
pub mod appendages;
pub mod bodytemp;
pub mod growthpattern;
pub mod sex;
pub mod senses;
pub mod intelligence;
pub mod lifespan;
pub mod organization;

pub struct Life {
    base: LifeBase,
    habitat: Habitat,
    trophiclevel: TrophicLevel,
    locomotion: Locomotion,
    size_category: SizeCategory,
    size: Size,
    symmetry: Symmetry,
    limbs: NumberOfLimbs,
    tails: Vec<Tail>,
    manipulators: Vec<Manipulator>,
    skeleton: Skeleton,
    breathing: Option<Breathing>,
    temperature_regulation: TemperatureRegulation,
    growth_pattern: GrowthPattern,
    reproduction: Reproduction,
    senses: Senses,
    lifespan: Lifespan,
    intelligence: Intelligence,
    mating_behavior: MatingBehavior,
    social_organization: SocialOrganization,
}

impl Life {
    pub fn random(
        location: Option<OrbitElement>,
        sapient: bool,
        gasgiant_dweller: bool,
        local_gravity: f64,
    ) -> Life {
        let base = LifeBase::random();
        let habitat = Habitat::random(location.as_ref());
        let trophiclevel = TrophicLevel::random(sapient, &habitat);
        let locomotion = Locomotion::random(&habitat, &trophiclevel, gasgiant_dweller);
        let size_category = SizeCategory::random(&base, &habitat, &trophiclevel, &locomotion, local_gravity);
        let size = Size::random(&size_category, &base, &habitat, &locomotion, local_gravity);
        let symmetry = Symmetry::random(&habitat, &locomotion);
        let num_of_limbs = NumberOfLimbs::random(&symmetry);
        let tails = Tail::random(&habitat, &symmetry);
        let manipulators = Manipulator::random(gasgiant_dweller, sapient, &num_of_limbs, &habitat, &trophiclevel, &locomotion);
        let skeleton = Skeleton::random(&size_category, &habitat, &locomotion, &symmetry, local_gravity);
        let breathing = Breathing::random(&habitat, &locomotion);
        let temperature_regulation = TemperatureRegulation::random(&habitat, &size_category, breathing.as_ref());
        let growth_pattern = GrowthPattern::random(&skeleton, &size_category, &locomotion);
        let reproduction = Reproduction::random(&habitat, &symmetry, &trophiclevel, &locomotion, &size_category, breathing.as_ref(), &temperature_regulation);
        let senses = Senses::random(local_gravity, &base, &size_category, &symmetry, &habitat, &trophiclevel, &locomotion, &skeleton, &reproduction);
        let lifespan = Lifespan::random(&base, &size, sapient);
        let intelligence = Intelligence::random(sapient, &size_category, &trophiclevel, &reproduction, &lifespan);
        let mating_behavior = MatingBehavior::random(&reproduction);
        let social_organization = SocialOrganization::random(&size_category, &trophiclevel, &mating_behavior);

        Life {
            base, habitat, trophiclevel,
            locomotion, size_category,
            size, symmetry, limbs: num_of_limbs,
            tails, manipulators, skeleton,
            breathing, temperature_regulation,
            growth_pattern, reproduction,
            senses, lifespan, intelligence,
            mating_behavior, social_organization,
        }
    }
}
