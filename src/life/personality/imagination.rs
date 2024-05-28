use dice::DiceExt;

use crate::{disadvantages::{hidebound::Hidebound, orh::nonstopideafactory::NonstopIdeaFactory, Disadvantage}, life::{sex::{reprstrategy::ReproductionStrategy, Reproduction}, trophiclevel::{Carnivore, Herbivore, TrophicLevel, TrophicLevelType}}, quirks::{dreamer::Dreamer, dull::Dull, imaginative::Imaginative, versatile::Versatile}};

use super::{PersonalityEffect, PersonalityEffectLevel};

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

impl PersonalityEffectLevel for Imagination {
    fn level(&self) -> i32 {
        match self {
            Self::Hidebound(true) => -3,
            Self::Hidebound(_) => -2,
            Self::Dull => -1,
            Self::Normal => 0,
            Self::Imaginative(x) => *x,
        }
    }
}

impl PersonalityEffect for Imagination {
    fn gain(&self, personality: &super::Personality, trophiclevel: &TrophicLevel) -> (Vec<Box<dyn crate::disadvantages::Disadvantage>>, Vec<Box<dyn crate::advantages::Advantage>>) {
        let _ = trophiclevel;
        let mut disadvs: Vec<Box<dyn Disadvantage>> = vec![];

        match self {
            Self::Imaginative(3) => {
                disadvs.push(Box::new(Imaginative));
                if personality.empathy.level() < 1 {
                    disadvs.push(Box::new(NonstopIdeaFactory))
                } else if personality.egoism.level() > 0 || personality.concentration.level() < 1 {
                    disadvs.push(Box::new(Dreamer))
                }
            },
            Self::Imaginative(2) => {
                disadvs.push(Box::new(Imaginative));
                if personality.egoism.level() > 0 || personality.concentration.level() < 1 {
                    disadvs.push(Box::new(Dreamer))
                }
            },
            Self::Imaginative(1) => {
                disadvs.push(Box::new(Imaginative));
                if personality.concentration.level() >= 0 && personality.egoism.level() < 2 {
                    disadvs.push(Box::new(Versatile))
                }
            },
            Self::Dull => disadvs.push(Box::new(Dull)),
            Self::Hidebound(with_iq_penalty) => disadvs.push(Box::new(Hidebound::new(*with_iq_penalty))),
            _ => ()
        }

        (disadvs, vec![])
    }
}
