//! Earthlings tend to have two sexes, but that's not anything universal
//! (not even among some species on our own planet).

use std::ops::{Deref, RangeInclusive};

use dicebag::{DiceExt, lo};
use serde::{Deserialize, Serialize};

use crate::life::{bodyplan::Symmetry, breathing::Breathing, habitat::Habitat, locomotion::Locomotion, regulation::TemperatureRegulation, size::Size, trophics::TrophicLevel};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reproduction {
    pub arrangement: SexualArrangement,
    pub gestation: Gestation,
    pub strategy: ReproductionStrategy,
} impl Reproduction {
    pub fn random(
        habitat: Habitat,
        size: Size,
        trophics: TrophicLevel,
        symmetry: Symmetry,
        locomotion: Locomotion,
        tr: TemperatureRegulation,
        breathing: Option<Breathing>,
    ) -> Self {
        let arrangement = SexualArrangement::random(trophics, symmetry, locomotion);
        let gestation = Gestation::random(habitat, locomotion, breathing, tr);
        let strategy = ReproductionStrategy::random(size, &gestation);
        Self {
            arrangement,
            gestation,
            strategy,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SexualArrangement {
    Asexual,
    Hermaphrodite,
    Parthenogenesis,
    Numbered { num: u8 },
    CycleMaleFemale,

    AlternatingOrConditional (Vec<SexualArrangement>),
} impl SexualArrangement {
    pub(crate) fn random(trophics: TrophicLevel, symmetry: Symmetry, locomotion: Locomotion) -> Self {
        use SexualArrangement as S;
        let mut modf = if locomotion.is_empty() {-1} else {0};
        if matches!(symmetry, Symmetry::Asymmetric) {
            modf -= 1;
        }
        if trophics.contains(TrophicLevel::AUTOTROPH) {
            modf -= 1;
        }

        fn arr(modf: i32) -> S {
            match 2.d6() + modf {
                ..=4 => if lo!() { S::Asexual } else { S::Parthenogenesis },
                ..=6 => S::Hermaphrodite,
                ..=9 => S::Numbered { num: 2 },
                10   => S::CycleMaleFemale,
                11   => S::Numbered { num:
                    match 1.d6() {
                        ..=3 => 3,
                        4|5  => 4,
                        _    => 2.d6()
                    } },
                _ => {
                    let mut s = vec![];
                    let s1 = arr(modf);
                    let s2 = arr(modf);
                    match (s1, s2) {
                        (S::AlternatingOrConditional(v1), S::AlternatingOrConditional(v2)) => {
                            s.extend(v1);
                            s.extend(v2);
                        },
                        (S::AlternatingOrConditional(v1), x) |
                        (x, S::AlternatingOrConditional(v1)) => {
                            s.extend(v1);
                            s.push(x);
                        },
                        (a, b) => {
                            s.push(a);
                            s.push(b);
                        }
                    }
                    S::AlternatingOrConditional(s)
                }
            }
        }

        arr(modf)
    }

    #[inline]
    pub fn is_sexual(&self) -> bool {
        match self {
            Self::CycleMaleFemale |
            Self::Numbered { .. } => true,
            Self::AlternatingOrConditional(v) => {
                for a in v {
                    if a.is_sexual() { return true }
                }
                false },
            _ => false
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum SpecialGestation {
    BroodParasite,
    ParasiticYoung,
    CannibalisticYoung { fatal_to_parent: bool },
} impl SpecialGestation {
    fn random() -> Self {
        match 1.d6() {
            1   => Self::BroodParasite,
            2|3 => Self::ParasiticYoung,
            4|5 => Self::CannibalisticYoung { fatal_to_parent: false },
            _   => Self::CannibalisticYoung { fatal_to_parent: true }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Gestation {
    EggLaying,
    LiveBearing { pouch: bool },
    Pollination,
    Spawning,
    Special { method: Box<Self>, special: SpecialGestation },
} impl Gestation {
    pub(crate) fn random(habitat: Habitat, locomotion: Locomotion, br: Option<Breathing>, tr: TemperatureRegulation) -> Self {
        let mut modf = if matches!(tr, TemperatureRegulation::WarmBlooded { .. }) {1} else {0};
        if locomotion.is_empty() {
            modf -= 2;
        }
        if matches!(habitat, Habitat::Water(_)) ||
            match br {
                Some(b) if matches!(b, Breathing::Amphibian) => true,
                _ => false
            }
        {
            modf -= 1;
        }
        let method = match 2.d6() + modf {
            ..=6 => if lo!() { Self::Spawning } else { Self::Pollination },
            7|8  => Self::EggLaying,
            9|10 => Self::LiveBearing { pouch: false },
            _    => Self::LiveBearing { pouch: true }
        };
        if 2.d6() == 12 {
            Self::Special { method: Box::new(method), special: SpecialGestation::random() }
        } else {
            method
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ReproductionStrategy {
    K { strong: bool, num_offspring: RangeInclusive<u8> },
    M { num_offspring: RangeInclusive<u8> },
    R { strong: bool, num_offspring: RangeInclusive<u8> },
} impl ReproductionStrategy {
    pub(crate) fn random(size: Size, gestation: &Gestation) -> Self {
        let mut modf = 0;
        let (spawning, add) = match gestation {
            Gestation::Spawning => (true, 2),
            Gestation::Special { method, .. } => match method.deref() {
                Gestation::Spawning => (true, 2),
                _ => (false, 0)
            },
            _ => (false, 0)
        };
        modf += add + match size {
            Size::Small { .. } => 1,
            Size::Large { .. } => -2,
            _ => 0
        };
        let minmax = |s:bool, min: u8, max:u8| {
            if !s { min..=max }
            else { (20 * min)..=(120 * max) }
        };
        match 2.d6() + modf {
            ..=4 => Self::K { strong: true, num_offspring: minmax(spawning, 1, 1) },
            5|6  => Self::K { strong: false, num_offspring: minmax(spawning, 1, 2) },
            7    => Self::M { num_offspring: minmax(spawning, 1, 1.d6()) },
            8|9  => Self::R { strong: false, num_offspring: minmax(spawning, 2, 7) },
            _    => Self::R { strong: true, num_offspring: minmax(spawning, 2, 12) }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum MatingBehavior {
    MatingOnly,
    Pair { temporary: bool },
    Harem,
    Hive,
} impl MatingBehavior {
    pub fn random(reproduction: &Reproduction) -> Self {
        let mut modf = match reproduction.strategy {
            ReproductionStrategy::K { strong: true, .. } => 1,
            ReproductionStrategy::R { strong: true, .. } => -1,
            _ => 0
        };
        modf += match reproduction.gestation {
            Gestation::LiveBearing { .. } => 1,
            Gestation::Spawning    |
            Gestation::Pollination => -1,
            _ => 0
        };
        modf += match reproduction.arrangement {
            SexualArrangement::Hermaphrodite => -2,
            _ => 0
        };
        match 2.d6() + modf {
            ..=5 => Self::MatingOnly,
            6|7  => Self::Pair { temporary: true },
            8    => Self::Pair { temporary: false },
            9|10 => Self::Harem,
            _    => Self::Hive
        }
    }
}
