use super::{sex::{reprstrategy::ReproductionStrategy, Reproduction}, size::SizeCategory, trophiclevel::{Herbivore, TrophicLevel, TrophicLevelType}};

pub enum Intelligence {
    Mindless,
    Preprogrammed,
    BestialLowIQ(i32),
    BestialHighIQ(i32),
    Presapient,
    Sapient(i32)
}

impl Intelligence {
    pub fn random(sapient: bool, size_category: &SizeCategory, trophiclevel: &TrophicLevel, reproduction: &Reproduction, lifespan: i32) -> Intelligence {
        let modifier
         = if trophiclevel.is_autotroph()
           || trophiclevel.is(TrophicLevelType::FilterFeeder)
           || trophiclevel.is(TrophicLevelType::Herbivore(Herbivore::Grazing))
           { -1 }
           else if trophiclevel.is(TrophicLevelType::Omnivore)
           ||      trophiclevel.is(TrophicLevelType::Herbivore(Herbivore::Gathering))
           { 1 } else { 0 }
         + if match size_category {
             SizeCategory::Small => -1,
             _ => 0
           } { -1 } else { 0 }
         + match reproduction.strategy() {
             ReproductionStrategy::StrongR(_) => -1,
             ReproductionStrategy::StrongK(_) => 1,
             _ => 0
        };

        
    }
}
