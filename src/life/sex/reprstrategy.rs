use dice::DiceExt;

use crate::life::size::SizeCategory;

use super::gestation::Gestation;

#[derive(PartialEq)]
pub enum ReproductionStrategy {
    StrongK(i32),
    ModerateK(i32),
    Median(i32),
    ModerateR(i32),
    StrongR(i32),
}

impl ReproductionStrategy {
    pub fn random(size_category: &SizeCategory, gestation: &Gestation) -> ReproductionStrategy {
        let modifier = match size_category {
            SizeCategory::Large => -2,
            SizeCategory::Small => 1,
            _ => 0
        } + match gestation {
            Gestation::Spawning(_) => 2,
            _ => 0
        };
        let spawn_mul = match gestation {
            Gestation::Spawning(_) => 2.d6() * 10,
            _ => 1
        };

        match 2.d6() + modifier {
            ..=4 => Self::StrongK(spawn_mul),
            5|6 => Self::ModerateK(1.d2() * spawn_mul),
            7 => Self::Median(1.d6() * spawn_mul),
            8|9 => Self::ModerateR((1.d6() + 1) * spawn_mul),
            10.. => Self::StrongR(2.d6() * spawn_mul)
        }
    }
}
