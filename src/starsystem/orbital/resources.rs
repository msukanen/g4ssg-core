use dice::DiceExt;

use super::{asteroidbelt::AsteroidBelt, planet::terrestrial::Terrestrial};

pub enum RVM {
    Worthless,
    VeryScant,
    Scant,
    VeryPoor,
    Poor,
    Average,
    Abundant,
    VeryAbundant,
    Rich,
    VeryRich,
    Motherlode,
}

impl From<&AsteroidBelt> for RVM {
    fn from(value: &AsteroidBelt) -> Self {
        let _ = value;
        match 3.d6() {
            ..=3 => Self::Worthless,
            4 => Self::VeryScant,
            5 => Self::Scant,
            6|7 => Self::VeryPoor,
            8|9 => Self::Poor,
            10|11 => Self::Average,
            12|13 => Self::Abundant,
            14|15 => Self::VeryAbundant,
            16 => Self::Rich,
            17 => Self::VeryRich,
            18.. => Self::Motherlode,
        }
    }
}

impl From<&Terrestrial> for RVM {
    fn from(value: &Terrestrial) -> Self {
        let _ = value;
        match 3.d6() {
            ..=2 => Self::Scant,
            3|4 => Self::VeryPoor,
            5..=7 => Self::Poor,
            8..=13 => Self::Average,
            14..=16 => Self::Abundant,
            17|18 => Self::VeryAbundant,
            19.. => Self::Rich,
        }
    }
}

impl RVM {
    pub fn modifier(&self) -> i32 {
        match self {
            Self::Worthless  => -5,
            Self::VeryScant  => -4,
            Self::Scant      => -3,
            Self::VeryPoor   => -2,
            Self::Poor       => -1,
            Self::Average    =>  0,
            Self::Abundant   =>  1,
            Self::VeryAbundant=> 2,
            Self::Rich       =>  3,
            Self::VeryRich   =>  4,
            Self::Motherlode =>  5
        }
    }
}
