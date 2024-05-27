use dice::DiceExt;

use super::{lifespan::Lifespan, sex::{reprstrategy::ReproductionStrategy, Reproduction}, size::SizeCategory, trophiclevel::{Herbivore, TrophicLevel, TrophicLevelType}};

pub enum Intelligence {
    Mindless,
    Preprogrammed,
    BestialLowIQ(i32),
    BestialHighIQ(i32),
    Presapient,
    Sapient(i32)
}

impl Intelligence {
    pub fn random(
        sapient: bool,
        size_category: &SizeCategory,
        trophiclevel: &TrophicLevel,
        reproduction: &Reproduction,
        lifespan: &Lifespan
    ) -> Intelligence {
        let modifier
         = if trophiclevel.is_autotroph()
           || trophiclevel.is(TrophicLevelType::FilterFeeder)
           || trophiclevel.is(TrophicLevelType::Herbivore(Herbivore::Grazing))
           { -1 }
           else if trophiclevel.is(TrophicLevelType::Omnivore)
           ||      trophiclevel.is(TrophicLevelType::Herbivore(Herbivore::Gathering))
           { 1 } else { 0 }
         + if match size_category {
             SizeCategory::Small => true,
             _ => false
           } { -1 } else { 0 }
         + match reproduction.strategy() {
             ReproductionStrategy::StrongR(_) => -1,
             ReproductionStrategy::StrongK(_) => 1,
             _ => 0
        } + match lifespan {
            Lifespan::Humanlike |
            Lifespan::Long(_)   |
            Lifespan::Undying   => 1,
            _ => 0
        };

        if !sapient {
            match 2.d6() + modifier {
                ..=3 => Self::Mindless,
                4|5 => Self::Preprogrammed,
                6..=8 => Self::BestialLowIQ(1.d3()),
                9|10 => Self::BestialHighIQ(1.d3() + 2),
                11.. => Self::Presapient
            }
        } else { Self::Sapient(1.d6() + 5) }

    }
}
