use std::collections::HashSet;

use derivative::Derivative;
use dice::{low, DiceExt, HiLo};

use crate::orbital::planet::climate::Climate;

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
pub enum TrophicLevel {
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

impl TrophicLevel {
    pub fn random(sapient: bool, climate: &Climate) -> HashSet<TrophicLevel> {
        fn select(sapient: bool, climate: &Climate) -> Vec<TrophicLevel> {
            let mut tlevels = vec![];
            
            let r = 3.d6();
            if r == 3 {
                tlevels.extend(select(sapient, climate));
                tlevels.extend(select(sapient, climate));
            } else {
                tlevels.push(if sapient {
                    match r {
                        ..=4 => if low!() {TrophicLevel::Parasite} else {TrophicLevel::Symbiont},
                        5 => match climate {
                            Climate::Arctic |
                            Climate::Desert => TrophicLevel::Carnivore(Carnivore::Trapping),
                            _ => TrophicLevel::FilterFeeder
                        },
                        6 => TrophicLevel::Carnivore(Carnivore::Pouncing),
                        7 => TrophicLevel::Scavenger,
                        8|9 => TrophicLevel::Herbivore(Herbivore::Gathering),
                        10 => TrophicLevel::Omnivore,
                        11|12 => TrophicLevel::Carnivore(Carnivore::Chasing),
                        13 => TrophicLevel::Herbivore(Herbivore::Grazing),
                        14 => TrophicLevel::Carnivore(Carnivore::Hijacking),
                        15|16 => TrophicLevel::Carnivore(Carnivore::Trapping),
                        17 => TrophicLevel::Decomposer,
                        _ => TrophicLevel::Autotroph(Autotroph::random())
                    }
                } else {
                    match r {
                        ..=4 => TrophicLevel::Autotroph(Autotroph::random()),
                        5 => TrophicLevel::Decomposer,
                        6 => TrophicLevel::Scavenger,
                        7 => TrophicLevel::Omnivore,
                        8|9 => TrophicLevel::Herbivore(Herbivore::Gathering),
                        10|11 => TrophicLevel::Herbivore(if low!() {Herbivore::Grazing} else {Herbivore::Browsing}),
                        12 => TrophicLevel::Carnivore(Carnivore::Pouncing),
                        13 => TrophicLevel::Carnivore(Carnivore::Chasing),
                        14 => TrophicLevel::Carnivore(Carnivore::Trapping),
                        15 => TrophicLevel::Carnivore(Carnivore::Hijacking),
                        16 => match climate {
                            Climate::Arctic |
                            Climate::Desert => TrophicLevel::Carnivore(Carnivore::Trapping),
                            _ => TrophicLevel::FilterFeeder
                        },
                        _ => if low!() {TrophicLevel::Parasite} else {TrophicLevel::Symbiont}
                    }
                })
            }

            tlevels
        }

        HashSet::from_iter(select(sapient, climate))
    }
}
