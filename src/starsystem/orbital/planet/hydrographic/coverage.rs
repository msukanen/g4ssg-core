use dice::{DiceExt, PercentageVariance};
use rand::Rng;

use crate::{maxof, starsystem::orbital::planet::terrestrial::{terratype::TerraType, worldtype::WorldType}};

#[derive(Clone)]
pub enum HydrographicCoverage {
    Barren,
    Percentage(f64)
}

impl HydrographicCoverage {
    pub fn random(terratype: &TerraType) -> HydrographicCoverage {
        match terratype {
            TerraType::Small(WorldType::Ice)       => Self::Percentage(rand::thread_rng().gen_range(28.5..=81.5)),
            TerraType::Medium(WorldType::Ice)      |
            TerraType::Large(WorldType::Ice)       => Self::Percentage(maxof!(0.0, 10.0 * (2.d6() - 10) as f64).delta(5)),
            TerraType::Medium(WorldType::Ammonia)  |
            TerraType::Large(WorldType::Ammonia)   |
            TerraType::Medium(WorldType::Ocean)    |
            TerraType::Medium(WorldType::Garden)   => Self::Percentage(rand::thread_rng().gen_range(47.5..=100.0)),
            TerraType::Large(WorldType::Ocean)     |
            TerraType::Large(WorldType::Garden)    => Self::Percentage(rand::thread_rng().gen_range(66.5..=100.0)),
            TerraType::Medium(WorldType::Greenhouse)|
            TerraType::Large(WorldType::Greenhouse)=> Self::Percentage(maxof!(0.0, 10.0 * (2.d6() - 7) as f64).delta(5)),
            _ => Self::Barren,
        }
    }
}
