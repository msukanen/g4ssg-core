use dice::DiceExt;

use crate::minof;

use super::terrestrial::{terratype::TerraType, worldtype::WorldType};

pub enum HydrographicCoverage {
    Barren,
    Percentage(f64)
}

impl HydrographicCoverage {
    pub fn random(terratype: TerraType) -> HydrographicCoverage {
        match terratype {
            TerraType::Tiny(WorldType::Rock)    |
            TerraType::Tiny(WorldType::Ice)     |
            TerraType::Small(WorldType::Hadean) |
            TerraType::Medium(WorldType::Hadean)|
            TerraType::Tiny(WorldType::Sulfur)  |
            TerraType::Small(WorldType::Rock)  => Self::Barren,
            TerraType::Small(WorldType::Ice)   => Self::Percentage(10.0 * (1.d6()+2) as f64),
            TerraType::Medium(WorldType::Ammonia)|
            TerraType::Large(WorldType::Ammonia) => Self::Percentage(minof!(100.0, 10.0 * 2.d6() as f64)),
        }
    }
}
