//! Stellar Age

use std::cmp::Ordering;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum StellarAge {
    E1,
    Y1(f64),
    I1(f64),
    O1(f64),
    I2(f64),
    E2(f64)
} impl StellarAge {
    pub fn random() -> Self {
        fn step(v: f64) -> f64 { v * (1.d6() - 1) as f64 }
        match 3.d6() {
            ..=3  => Self::E1,
            ..=6  => Self::Y1( 0.1 + step(0.3) + step(0.05)),
            ..=10 => Self::I1( 2.0 + step(0.6) + step(0.1)),
            ..=14 => Self::O1( 5.6 + step(0.6) + step(0.1)),
            ..=17 => Self::I2( 8.0 + step(0.6) + step(0.1)),
            _     => Self::E2(10.0 + step(0.6) + step(0.1))
        }
    }

    pub fn gyr(&self) -> f64 {
        match self {
            StellarAge::E2(v) |
            StellarAge::I1(v) |
            StellarAge::I2(v) |
            StellarAge::O1(v) |
            StellarAge::Y1(v) => *v,
            StellarAge::E1 => 0.0
        }
    }
}

impl Eq for StellarAge {}
impl Ord for StellarAge {
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