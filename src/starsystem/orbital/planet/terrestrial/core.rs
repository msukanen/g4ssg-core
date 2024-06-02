use dice::DiceExt;
use rand::Rng;

use super::{terratype::TerraType, worldtype::WorldType};

#[derive(Clone)]
pub(crate) enum Core {
    Icy(f64),
    SmallIron(f64),
    LargeIron(f64)
}

impl Core {
    pub fn random(terratype: &TerraType) -> Core {
        match terratype {
            TerraType::Tiny(WorldType::Ice)      |
            TerraType::Tiny(WorldType::Sulfur)   |
            TerraType::Small(WorldType::Hadean)  |
            TerraType::Small(WorldType::Ice)     |
            TerraType::Medium(WorldType::Hadean) |
            TerraType::Medium(WorldType::Ammonia)|
            TerraType::Large(WorldType::Ammonia) => Core::Icy(rand::thread_rng().gen_range(0.25..=0.75)),
            TerraType::Tiny(WorldType::Rock)     |
            TerraType::Small(WorldType::Rock)    => Core::SmallIron(rand::thread_rng().gen_range(0.55..=1.05)),
            _                                    => Core::LargeIron(rand::thread_rng().gen_range(0.75..=1.25))
        }
    }

    pub fn density(&self) -> f64 {
        match self {
            Core::Icy(d)       |
            Core::SmallIron(d) |
            Core::LargeIron(d) => *d
        }
    }
}
