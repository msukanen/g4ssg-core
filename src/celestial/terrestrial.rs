//! Terrestrial Planets, Planetoids, Moons, etc.

use dicebag::{DiceExt, lo};
use either::Either;
use mshc::Named;
use serde::{Deserialize, Serialize};

use crate::{UNNAMED, celestial::{Atmosphere, ab::{ABRegion, AsteroidBeltType}, hydrocover::Hydrocover, orbital::OrbitContent}};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SizeCategory {
    /// Not really a planet at all; a literal *cluster* of asteroids more or less tightly packed.
    AsteroidCluster,
    /// A tiny planet/planetoid. In some contexts synonymous with 'Moon'.
    Tiny,
    /// Mercury, etc.
    Small,
    /// Earth, Venus, etc.
    Medium,
    /// E.g. Super-Earths, etc.
    Large,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum TerrestrialOverallType {
    Hostile,
    Barren,
    Garden
} impl TerrestrialOverallType {
    fn random() -> Self {
        match 3.d6() {
            ..=7  => Self::Hostile,
            ..=13 => Self::Barren,
            _     => Self::Garden
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum TerrestrialSubType {
    Ammonia,
    Chthonian,
    Garden,
    Greenhouse,
    Hadean,
    Ice,
    Ocean,
    Rock,
    Sulfur,
}

#[derive(Debug, Clone, Deserialize, Serialize, Named)]
pub struct Terrestrial {
    name: String,
    overall: TerrestrialOverallType,
    sub: TerrestrialSubType,
    size: SizeCategory,
    atm: Option<Atmosphere>,
    hydrocover: Option<f64>,
}

impl Terrestrial {
    /// Generate a random terrestrial planet(oid)… or an asteroid belt if things so judge.
    pub fn random() -> Either<Self, OrbitContent> {
        use TerrestrialOverallType as O;
        use TerrestrialSubType as S;
        use SizeCategory::*;

        let overall = O::random();
        let (sub, size) = match (3.d6(), &overall) {
            (..=4, O::Hostile) => (S::Chthonian, Medium),
            (..=6, O::Hostile) => (S::Greenhouse, Medium),
            (..=3, O::Barren) => (S::Hadean, Small),
            (   4, O::Barren) => (S::Ice, Small),
            (..=6, O::Barren) => (S::Rock, Small),
            (..=16, O::Garden) => (S::Garden, Medium),
            (..=9, O::Hostile) => (S::Sulfur, Tiny),
            (..=8, O::Barren)  => (S::Rock, Tiny),
            (..=10, O::Barren) => (S::Ice, Tiny),
            (..=12, O::Hostile) => (S::Ammonia, Medium),
            (..=12, O::Barren)  => return Either::Right(OrbitContent::AB(AsteroidBeltType::random(ABRegion::Mid, None))),
            (..=14, O::Hostile) => (S::Ammonia, Large),
            (..=14, O::Barren)  => (S::Ocean, Medium),
            (..=16, O::Hostile) => (S::Greenhouse, Large),
            (   15, O::Barren)  => (S::Ice, Medium),
            (   16, O::Barren)  => (S::Hadean, Medium),
            (    _, O::Hostile) => (S::Chthonian, Large),
            (   17, O::Barren) => (S::Ocean, Large),
            (    _, O::Barren) => (S::Ice, Large),
            _ => (S::Garden, Large)
        };

        Either::Left(Self::random_oss(overall, sub, size))
    }

    pub fn random_sized(size: SizeCategory) -> Self {
        use TerrestrialOverallType as O;
        use TerrestrialSubType as S;
        use SizeCategory as C;
        let mut overall = TerrestrialOverallType::random();
        while (overall == O::Garden && (size == C::Tiny || size == C::Small)) ||
              (overall == O::Hostile && size == C::Small)
        {
            overall = TerrestrialOverallType::random();
        }
        let r = 3.d6();
        let sub = match (3.d6(), &overall, size) {
            (_, _, C::AsteroidCluster) |
            (_, O::Garden, C::Tiny)    |
            (_, O::Garden, C::Small)   |
            (_, O::Hostile, C::Small)  => unreachable!(),
            
            (_,     O::Hostile, C::Tiny) => S::Sulfur,
            (..=9,  O::Barren,  C::Tiny) => S::Rock,
            (_,     O::Barren,  C::Tiny) => S::Ice,

            (..=5,  O::Barren,  C::Small) => S::Hadean,
            (..=8,  O::Barren,  C::Small) => S::Ice,
            (_,     O::Barren,  C::Small) => S::Rock,

            (6|7|8, O::Hostile, C::Medium) => S::Greenhouse,
            (..=16, O::Hostile, C::Medium) => S::Ammonia,
            (_,     O::Hostile, C::Medium) => S::Chthonian,

            (..=6,  O::Barren,  C::Medium) => S::Ice,
            (..=8,  O::Barren,  C::Medium) => S::Hadean,
            (_,     O::Barren,  C::Medium) => S::Ocean,

            (..=13, O::Hostile, C::Large) => S::Ammonia,
            (..=16, O::Hostile, C::Large) => S::Greenhouse,
            (_,     O::Hostile, C::Large) => S::Chthonian,

            (..=11, O::Barren,  C::Large) => S::Ocean,
            (_,     O::Barren,  C::Large) => S::Ice,

            (_,     O::Garden,  C::Medium) |
            (_,     O::Garden,  C::Large)  => S::Garden,            
        };

        Self::random_oss(overall, sub, size)
    }

    fn random_oss(overall: TerrestrialOverallType, sub: TerrestrialSubType, size: SizeCategory) -> Self {
        let atm = Atmosphere::random(sub.into(), size.into());
        let hydrocover = Hydrocover::random(atm.as_ref(), sub.into(), size.into());

        Self {
            name: UNNAMED.into(),
            overall,
            sub,
            size,
            atm,
            hydrocover,
        }
    }
}
