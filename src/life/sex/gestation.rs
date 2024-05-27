use dice::{DiceExt, HiLo};

use crate::life::{bodytemp::TemperatureRegulation, breathing::Breathing, habitat::Habitat, locomotion::Locomotion};

pub enum SpecialGestationMethod {
    BroodParasite,
    ParasiticYoung,
    CannibalisticYoungImplant,
    CannibalisticYoungEatEachOther,
}

impl SpecialGestationMethod {
    pub fn random() -> SpecialGestationMethod {
        match 1.d6() {
            ..=1 => Self::BroodParasite,
            2|3 => Self::ParasiticYoung,
            4|5 => Self::CannibalisticYoungImplant,
            6.. => Self::CannibalisticYoungEatEachOther
        }
    }
}

pub enum Gestation {
    Spawning(Option<SpecialGestationMethod>),
    Pollinating(Option<SpecialGestationMethod>),
    EggLaying(Option<SpecialGestationMethod>),
    LiveBearing(Option<SpecialGestationMethod>, bool),//false = pouch
}

impl Gestation {
    pub fn random(habitat: &Habitat, locomotion: &Locomotion, breathing: Option<&Breathing>, temperature_regulation: &TemperatureRegulation) -> Gestation {
        let special_method = match 2.d6() {
            ..=11 => None,
            _ => Some(SpecialGestationMethod::random())
        };
        let modifier
         = if habitat.is_aquatic() {-1} else {0}
         + match breathing {
            Some(Breathing::GillsAndLungs) => -1,
            _ => 0
         }
         + if locomotion.is_immobile() {-2} else {0}
         + match temperature_regulation {
            TemperatureRegulation::WarmBlooded(_) => 1,
            _ => 0
         };
        match 2.d6() + modifier {
            ..=6 => if 1.d2().lo() {Self::Spawning(special_method)} else {Self::Pollinating(special_method)},
            7|8 => Self::EggLaying(special_method),
            9|10 => Self::LiveBearing(special_method, true),
            11.. => Self::LiveBearing(special_method, false)
        }
    }

    pub fn is_spawning(&self) -> bool {
        match self {
            Self::Spawning(_) => true,
            _ => false
        }
    }
}
