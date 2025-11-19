//! Stellar Population
//! 
//! Populations I and II, plus theoretical III.
use std::cmp::Ordering;

use dicebag::{DiceExt, PercentageVariance};
use serde::{Deserialize, Serialize};

const AGE_OF_UNIVERSE_GYR: f64 = 13.813;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum StellarPopulation {
    /// Extremely young population-I — coincidentally also covers existing superstars.
    E1,
    /// Young population-I ~100Myr to ~1.9Gyr
    Y1(f64),
    /// Intermediate population-I ~2Gyr to ~7.5Gyr
    I1(f64),
    /// Old population-I ~5.6Gyr to ~9.1Gyr
    O1(f64),
    /// Intermediate population-II ~8Gyr to ~11.5Gyr
    I2(f64),
    /// Extreme population-II ~10Gyr to ~13.5Gyr (near the approximate age of the universe, as we know it)
    E2(f64),
    /// Immensely old population-III — too rare to come up with a random roll.
    // Age value not present as by nature they're almost exactly as old as the universe itself, if they actually exist at all.
    III
} impl StellarPopulation {
    pub fn random() -> Self {
        fn step(v: f64) -> f64 { v * (1.d6() - 1) as f64 }
        match 3.d6() {
            ..=3  => Self::E1,
            ..=6  => Self::Y1(( 0.1 + step(0.3) + step(0.05)).jitter_percentage(1.0)),
            ..=10 => Self::I1(( 2.0 + step(0.6) + step(0.1)).jitter_percentage(1.0)),
            ..=14 => Self::O1(( 5.6 + step(0.6) + step(0.1)).jitter_percentage(1.0)),
            ..=17 => Self::I2(( 8.0 + step(0.6) + step(0.1)).jitter_percentage(1.0)),
            _     => Self::E2((10.0 + step(0.6) + step(0.1)).jitter_percentage(1.0))
        }
    }

    pub fn gyr(&self) -> f64 {
        match self {
            StellarPopulation::E2(v) |
            StellarPopulation::I1(v) |
            StellarPopulation::I2(v) |
            StellarPopulation::O1(v) |
            StellarPopulation::Y1(v) => *v,
            StellarPopulation::E1 => 0.0,
            StellarPopulation::III => AGE_OF_UNIVERSE_GYR,
        }
    }
}

impl Eq for StellarPopulation {}
impl Ord for StellarPopulation {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::E1, Self::E1) => return Ordering::Equal,
            (_, Self::E1) => return Ordering::Greater,
            (Self::E1, _) => return Ordering::Less,
            _ => ()
        }

        let a = self.gyr();
        let b = other.gyr();

        if a < b - 0.0001 { Ordering::Less }
        else if a + 0.0001 > b { Ordering::Greater }
        else { Ordering::Equal }
    }
}

pub trait AsYears {
    fn as_years(&self) -> f64;
}

impl AsYears for StellarPopulation {
    fn as_years(&self) -> f64 {
        self.gyr() * 1_000_000_000.0
    }
}
