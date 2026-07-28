//! Social organization.

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{reproduction::MatingBehavior, size::Size, trophics::TrophicLevel};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SocialGroupMode {
    Troop,
    Pack,
    Herd,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum SocialOrganization {
    Solitary,
    PairBond,
    SmallGroup { members: u8, mode: SocialGroupMode },
    MedGroup { members: u8, mode: SocialGroupMode },
    LgHerd { members: u8 },
    Hive,
} impl SocialOrganization {
    pub fn random(size: Size, trophics: TrophicLevel, mating: MatingBehavior) -> Self {
        if matches!(mating, MatingBehavior::Hive) {
            return Self::Hive;
        }

        let mut modf =
            if trophics.contains(TrophicLevel::CARNIVORE)
                { -1 }
            else if trophics.contains(TrophicLevel::GRAZING)
                 { 1 }
            else { 0 };
        if matches!(size, Size::Large { .. }) {
            modf -= 1;
        }
        if matches!(mating, MatingBehavior::Harem) {
            modf += 1;
        }
        modf += match mating {
            MatingBehavior::Pair { .. } => 0,
            _ => -1
        };
        match 2.d6() + modf {
            ..=6 => Self::Solitary,
            7|8  => Self::PairBond,
            9|10 => Self::SmallGroup { members: 1.d6() + 4, mode: match 1.d6() {
                    ..=2 => SocialGroupMode::Troop,
                    3|4  => SocialGroupMode::Pack,
                    _    => SocialGroupMode::Herd
                }},
            11 => Self::MedGroup { members: 4.d6(), mode: match 1.d6() {
                    ..=2 => SocialGroupMode::Pack,
                    _    => SocialGroupMode::Herd
                }},
            _ => Self::LgHerd { members: 1.d6() * 10 }
        }
    }
}
