//! Terrestrial Planets, Planetoids, Moons, etc.

use dicebag::DiceExt;
use either::Either;
use mshc::Named;
use serde::{Deserialize, Serialize};

use crate::{UNNAMED, celestial::{ab::{ABRegion, AsteroidBeltType}, orbital::OrbitContent, rnd_atm_mass}};

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
    atm_mass: Option<f64>,
}

impl Terrestrial {
    /// Generate a random terrestrial planet(oid).
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

        let atm_mass = rnd_atm_mass(sub.into(), size.into());

        Either::Left(Self {
            name: UNNAMED.into(),
            overall,
            sub,
            size,
            atm_mass,
        })
    }
}
