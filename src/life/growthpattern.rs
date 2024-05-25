use dice::DiceExt;

use super::{locomotion::Locomotion, size::SizeCategory, skeleton::Skeleton};

pub enum GrowthPattern {
    Metamorphosis,
    Molting,
    Continuous,
    Unusual,
}

impl GrowthPattern {
    pub fn random(skeleton: &Skeleton, size_category: &SizeCategory, locomotion: &Locomotion) -> GrowthPattern {
        let modifier
         = if skeleton.is_exoskeleton() {-1} else {0}
         + match size_category {
             SizeCategory::Large => 1,
             _ => 0
         } + if locomotion.is_immobile() {1} else {0};
        match 2.d6() + modifier {
            ..=4 => GrowthPattern::Metamorphosis,
            5|6 => GrowthPattern::Molting,
            7..=11 => GrowthPattern::Continuous,
            12.. => GrowthPattern::Unusual
        }
    }
}
