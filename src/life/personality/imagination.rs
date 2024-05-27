use dice::DiceExt;

use crate::life::{sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::{Carnivore, Herbivore, TrophicLevel, TrophicLevelType}};

pub enum Imagination {
    Imaginative(i32),
    Normal,
    Dull,
    Hidebound(bool),
}

impl Imagination {
    pub fn random(trophiclevel: &TrophicLevel, reproduction: &Reproduction) -> Imagination {
        let modifier
         = if trophiclevel.is_carnivore(Some(Carnivore::Pouncing))
           || trophiclevel.is(TrophicLevelType::Omnivore)
           || trophiclevel.is_herbivore(Some(Herbivore::Gathering))
           { 1 }
           else if trophiclevel.is_autotroph()
           || trophiclevel.is(TrophicLevelType::FilterFeeder)
           || trophiclevel.is_herbivore(Some(Herbivore::Grazing))
           { -1 } else { 0 }
         + match reproduction.strategy() {
             ReproductionStrategy::StrongK(_) => 1,
             ReproductionStrategy::StrongR(_) => -1,
             _ => 0
         };
        match 1.d6() - 1.d6() + modifier {
            ..=-3 => Self::Hidebound(true),
            -2 => Self::Hidebound(false),
            -1 => Self::Dull,
            0 => Self::Normal,
            1 => Self::Imaginative(1),
            2 => Self::Imaginative(2),
            3.. => Self::Imaginative(3),
        }
    }
}
