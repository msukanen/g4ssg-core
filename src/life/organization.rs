use dice::DiceExt;

use super::{sex::mating::MatingBehavior, size::SizeCategory, trophiclevel::{Herbivore, TrophicLevel}};

pub enum GroupSize {
    Troop,
    Pack,
    Herd,
}

pub enum SocialOrganization {
    Solitary,
    PairBond,
    SmallGroup(i32, GroupSize),
    MediumGroup(i32, GroupSize),
    LargeHerd(i32),
    Hive,
}

impl SocialOrganization {
    pub fn random(size_category: &SizeCategory, trophiclevel: &TrophicLevel, mating_behavior: &MatingBehavior) -> SocialOrganization {
        if mating_behavior == &MatingBehavior::Hive {
            return Self::Hive;
        }

        let modifier
         = if trophiclevel.is_carnivore(None) {-1}
           else if trophiclevel.is_herbivore(Some(Herbivore::Grazing)) {1}
           else {0}
         + match mating_behavior {
             MatingBehavior::Harem => 1,
             MatingBehavior::Hive |
             MatingBehavior::MatingOnly => -1,
             _ => 0
         };
        match 2.d6() + modifier {
            ..=6 => Self::Solitary,
            7|8 => Self::PairBond,
            9|10 => Self::SmallGroup(2.d6(), match 1.d3() {
                1 => GroupSize::Troop,
                2 => GroupSize::Pack,
                _ => GroupSize::Herd
            }),
            11 => Self::MediumGroup(4.d6(), if 1.d3() == 1 {GroupSize::Pack} else {GroupSize::Herd}),
            12.. => Self::LargeHerd(1.d6() * 10)
        }
    }
}
