use dice::DiceExt;

use super::{arrangement::SexualArrangement, gestation::Gestation, reprstrategy::ReproductionStrategy, ArrangementCheck, Reproduction};

#[derive(PartialEq)]
pub enum MatingBehavior {
    MatingOnly,
    TemporaryPair,
    PermanentPair,
    Harem,
    Hive
}

impl MatingBehavior {
    pub fn random(reproduction: &Reproduction) -> MatingBehavior {
        let modifier
         = if reproduction.is(&SexualArrangement::Hermaphrodite) {-2} else {0}
         + match reproduction.gestation {
            Gestation::Spawning(_) => -1,
            Gestation::LiveBearing(_,_) => 1,
            _ => 0,
        } + match reproduction.strategy() {
            ReproductionStrategy::StrongK(_) => 1,
            ReproductionStrategy::StrongR(_) => -1,
            _ => 0
        };
        match 2.d6() + modifier {
            ..=5 => Self::MatingOnly,
            6|7 => Self::TemporaryPair,
            8 => Self::PermanentPair,
            9|10 => Self::Harem,
            11.. => Self::Hive
        }
    }
}
