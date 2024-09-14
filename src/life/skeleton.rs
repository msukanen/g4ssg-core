use dicebag::DiceExt;

use super::{habitat::Habitat, locomotion::{Locomotion, LocomotionMode}, size::SizeCategory, symmetry::Symmetry};

#[derive(PartialEq)]
pub enum SkeletonType {
    External,
    Hydrostatic,
    Internal,
}

pub struct  Skeleton {
    types: Vec<SkeletonType>
}

impl Skeleton {
    pub fn random(size_category: &SizeCategory, habitat: &Habitat, locomotion: &Locomotion, symmetry: &Symmetry, local_gravity: f64) -> Skeleton {
        let modifier = match size_category {
            SizeCategory::HumanScale => 1,
            SizeCategory::Large => 2,
            _ => 0
        } + match habitat {
            Habitat::Land(_) => 1,
            _ => 0
        } + if locomotion.is_immobile() || locomotion.is(LocomotionMode::Slithering) {
            -1
        } else {0} + match symmetry {
            Symmetry::Asymmetric => -1,
            _ => 0
        } + if local_gravity < 0.5 {-1}
        else if local_gravity > 1.25 {1}
        else {0};
        fn select(modifier: i32) -> Vec<SkeletonType> {
            let mut types = vec![];
            match 2.d6() + modifier {
                ..=3 => (),
                4|5 => types.push(SkeletonType::Hydrostatic),
                6|7 => types.push(SkeletonType::External),
                8..=10 => types.push(SkeletonType::Internal),
                _ => {
                    types.extend(select(modifier));
                    types.extend(select(modifier))
                }
            }
            types
        }
        Skeleton { types: select(modifier) }
    }

    pub fn is(&self, skeleton: SkeletonType) -> bool {
        self.types.contains(&skeleton)
    }

    pub fn is_exoskeleton(&self) -> bool {
        self.is(SkeletonType::External)
    }
}
