use dice::{DiceExt, HiLo};

use crate::life::{locomotion::Locomotion, symmetry::Symmetry, trophiclevel::TrophicLevel};

pub enum SexualArrangement {
    Asexual,
    Parthenogenesis,
    Hermaphrodite,
    TwoSexes,
    SwitchingMaleFemale,
    MultipleSexes(i32)
}

impl SexualArrangement {
    pub fn random(symmetry: &Symmetry, trophiclevel: &TrophicLevel, locomotion: &Locomotion) -> Vec<SexualArrangement> {
        let mut arrangements = vec![];
        let modifier
         = if locomotion.is_immobile() {-1} else {0}
         + if symmetry.is(&Symmetry::Asymmetric) {-1} else {0}
         + if trophiclevel.is_autotroph() {-1} else {0};
        match 2.d6() + modifier {
            ..=4 => arrangements.push(if 1.d2().lo() {Self::Asexual} else {Self::Parthenogenesis}),
            5|6 => arrangements.push(Self::Hermaphrodite),
            7..=9 => arrangements.push(Self::TwoSexes),
            10 => arrangements.push(Self::SwitchingMaleFemale),
            11 => arrangements.push(Self::MultipleSexes(match 1.d6() {
                ..=3 => 3,
                4|5 => 4,
                6.. => 2.d6()
            })),
            12.. => {
                arrangements.extend(Self::random(symmetry, trophiclevel, locomotion));
                arrangements.extend(Self::random(symmetry, trophiclevel, locomotion))
            }
        }
        arrangements
    }
}
