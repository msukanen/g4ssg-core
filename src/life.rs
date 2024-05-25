use crate::orbital::OrbitElement;

use self::{base::LifeBase, habitat::Habitat, locomotion::Locomotion, manipulators::Manipulator, numlimbs::NumberOfLimbs, size::{Size, SizeCategory}, symmetry::Symmetry, tail::Tail, trophiclevel::TrophicLevel};

pub mod base;
pub mod habitat;
pub mod trophiclevel;
pub mod locomotion;
pub mod size;
pub mod symmetry;
pub mod numlimbs;
pub mod tail;
pub mod manipulators;
pub mod skeleton;

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
    manipulators: Vec<Manipulator>
}

impl Life {
    pub fn random(
        location: Option<OrbitElement>,
        sapient: bool,
        gasgiant: bool
    ) -> Life {
        let base = LifeBase::random();
        let habitat = Habitat::random(location.as_ref());
        let trophiclevel = TrophicLevel::random(sapient, &habitat);
        let locomotion = Locomotion::random(&habitat, &trophiclevel, gasgiant);

        Life {
            base, habitat, trophiclevel, locomotion, size_category: (), size: (), symmetry: (), limbs: (), tails: (), manipulators: () }
    }
}
