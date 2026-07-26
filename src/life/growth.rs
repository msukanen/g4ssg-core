//! Growth patterns.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{bodyplan::{BodyPlan, Skeleton}, habitat::Habitat, locomotion::Locomotion, size::Size};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum GrowthPattern {
    Metamorphosis,
    Molting,
    Continuous,
    /// adding segments, branching, etc.
    Unusual,
} impl GrowthPattern {
    pub(crate) fn random(habitat: Habitat, size: Size, locomotion: Locomotion, bodyplan: &BodyPlan) -> Self {
        let mut modf = if matches!(size, Size::Large { .. }) {1} else {0};
        if locomotion.is_empty() {
            modf += 1;
        }
        if bodyplan.skeleton.is_some_and(|s| matches!(s, Skeleton::External|Skeleton::CombinationEI)) {
            modf -= 1;
        }
        match habitat {
            Habitat::Space(_) => match 2.d6() + modf {
                ..=5  => Self::Metamorphosis,
                ..=10 => Self::Continuous,
                _     => Self::Unusual
            }
            _ => match 2.d6() + modf {
                ..=4  => Self::Metamorphosis,
                5|6   => Self::Molting,
                ..=11 => Self::Continuous,
                _     => Self::Unusual
            }
        }
    }
}
