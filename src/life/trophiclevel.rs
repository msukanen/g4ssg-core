use std::collections::HashSet;

use derivative::Derivative;
use dice::{low, DiceExt, HiLo};

use crate::life::habitat::ArcticOrDesert;

use super::habitat::Habitat;

#[derive(Derivative)]
#[derivative(PartialEq, Eq, Hash)]
pub enum Autotroph {
    Photosynthetic,
    Chemosynthetic,
    Other
}

impl Autotroph {
    pub fn random() -> Autotroph {
        match 1.d6() {
            ..=3 => Self::Photosynthetic,
            4|5 => Self::Chemosynthetic,
            6.. => Self::Other
        }
    }
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, Hash)]
pub enum Herbivore {
    Gathering,
    Grazing,
    Browsing
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, Hash)]
pub enum Carnivore {
    Pouncing,
    Chasing,
    Trapping,
    Hijacking
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, Hash)]
pub enum TrophicLevelType {
    Autotroph(Autotroph),
    Decomposer,
    Scavenger,
    Omnivore,
    Herbivore(Herbivore),
    Carnivore(Carnivore),
    FilterFeeder,
    Parasite,
    Symbiont
}

pub struct TrophicLevel {
    levels: HashSet<TrophicLevelType>,
}

impl TrophicLevel {
    pub fn random(sapient: bool, habitat: &Habitat) -> TrophicLevel {
        fn select(sapient: bool, habitat: &Habitat) -> Vec<TrophicLevelType> {
            let mut tlevels = vec![];
            
            let r = 3.d6();
            if r == 3 {
                tlevels.extend(select(sapient, habitat));
                tlevels.extend(select(sapient, habitat));
            } else {
                tlevels.push(if sapient {
                    match r {
                        ..=4 => if low!() {TrophicLevelType::Parasite} else {TrophicLevelType::Symbiont},
                        5 => if habitat.is_arctic() || habitat.is_desert() {
                            TrophicLevelType::Carnivore(Carnivore::Trapping)
                        } else {
                            TrophicLevelType::FilterFeeder
                        },
                        6 => TrophicLevelType::Carnivore(Carnivore::Pouncing),
                        7 => TrophicLevelType::Scavenger,
                        8|9 => TrophicLevelType::Herbivore(Herbivore::Gathering),
                        10 => TrophicLevelType::Omnivore,
                        11|12 => TrophicLevelType::Carnivore(Carnivore::Chasing),
                        13 => TrophicLevelType::Herbivore(Herbivore::Grazing),
                        14 => TrophicLevelType::Carnivore(Carnivore::Hijacking),
                        15|16 => TrophicLevelType::Carnivore(Carnivore::Trapping),
                        17 => TrophicLevelType::Decomposer,
                        _ => TrophicLevelType::Autotroph(Autotroph::random())
                    }
                } else {
                    match r {
                        ..=4 => TrophicLevelType::Autotroph(Autotroph::random()),
                        5 => TrophicLevelType::Decomposer,
                        6 => TrophicLevelType::Scavenger,
                        7 => TrophicLevelType::Omnivore,
                        8|9 => TrophicLevelType::Herbivore(Herbivore::Gathering),
                        10|11 => TrophicLevelType::Herbivore(if low!() {Herbivore::Grazing} else {Herbivore::Browsing}),
                        12 => TrophicLevelType::Carnivore(Carnivore::Pouncing),
                        13 => TrophicLevelType::Carnivore(Carnivore::Chasing),
                        14 => TrophicLevelType::Carnivore(Carnivore::Trapping),
                        15 => TrophicLevelType::Carnivore(Carnivore::Hijacking),
                        16 => if habitat.is_arctic() || habitat.is_desert() {
                            TrophicLevelType::Carnivore(Carnivore::Trapping)
                        } else {
                            TrophicLevelType::FilterFeeder
                        },
                        _ => if low!() {TrophicLevelType::Parasite} else {TrophicLevelType::Symbiont}
                    }
                })
            }

            tlevels
        }

        TrophicLevel { levels: HashSet::from_iter(select(sapient, habitat)) }
    }

    pub fn is(&self, tt: TrophicLevelType) -> bool {
        self.levels.contains(&tt)
    }
}
