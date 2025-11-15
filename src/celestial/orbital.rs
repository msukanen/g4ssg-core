//! Orbital Stuff
//! 

use std::cmp::Ordering;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::unit::Metric;

/// Orbital eccentricity.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrbitalEccentricity {
    /// Eccentricity, generally \[0 → 0.95\].
    ecc: f64,
    /// Average distance.
    avg: Metric,
} impl OrbitalEccentricity {
    /// Generate random orbital eccentricity.
    /// 
    /// # Args
    /// 
    /// * `osep`— separation to the nearest relevant neighbor.
    pub fn random(osep: &OrbitalSeparation) -> Self {
        let m = match osep {
            OrbitalSeparation::VC(_) => -6,
            OrbitalSeparation::C(_) => -4,
            OrbitalSeparation::M(_) => -2,
            _ => 0
        };

        Self {
            ecc: match 3.d6() + m {
                ..=3 => 0.0,
                4 => 0.1,
                5 => 0.2,
                6 => 0.3,
                ..=8 => 0.4,
                ..=11 => 0.5,
                ..=13 => 0.6,
                ..=15 => 0.7,
                16 => 0.8,
                17 => 0.9,
                _ => 0.95
            },
            avg: osep.as_metric()
        }
    }

    /// Get the underlying eccentricity value.
    pub fn ecc(&self) -> f64 {
        self.ecc
    }

    /// Get average distance (in **au**, usually).
    pub fn avg_distance(&self) -> Metric {
        self.avg.clone()
    }

    /// Get minimum distance (periapsis; in **au**, usually).
    pub fn min_distance(&self) -> Metric {
        (1.0 - self.ecc) * &self.avg
    }

    /// Get maximum distance (apoapsis; in **au**, usually).
    pub fn max_distance(&self) -> Metric {
        (1.0 + self.ecc) * &self.avg
    }
}

/// Orbital separation decision methods…
pub enum OSDMethod {
    Basic,
    /// For distant companions' subcompanions.
    SC,
    /// For tertiary, quaternary, etc. stars.
    TOB,
} impl Default for OSDMethod {
    fn default() -> Self {
        Self::Basic
    }
}

/// Orbital separation of stars.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum OrbitalSeparation {
    /// Very close.
    VC(Metric),
    /// Close.
    C(Metric),
    /// Moderate.
    M(Metric),
    /// Wide.
    W(Metric),
    /// Distant.
    D(Metric)
} impl OrbitalSeparation {
    /// Generate random orbital separation.
    /// 
    /// # Args
    /// 
    /// `third_or_beyond`— set to `true` for other but primary and secondary star of a system.
    pub fn random(method: OSDMethod) -> Self {
        let base_au: Metric = 2.d6().into();
        match 3.d6() + match method {
            OSDMethod::Basic => 0,
            OSDMethod::TOB => 6,
            OSDMethod::SC => -6
        } {
            ..=6  => Self::VC(0.05 * base_au),
            ..=9  => Self::C(0.5 * base_au),
            ..=11 => Self::M(2 * base_au),
            ..=14 => Self::W(10 * base_au),
            _     => Self::D(50 * base_au)
        }
    }

    /// Get the separation, usually as **au**.
    pub fn as_metric(&self) -> Metric {
        match self {
            Self::C(v) |
            Self::D(v) |
            Self::M(v) |
            Self::W(v) |
            Self::VC(v) => v.clone()
        }
    }
}

impl Eq for OrbitalSeparation {}
impl Ord for OrbitalSeparation {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord =
        |x: &OrbitalSeparation| match x {
            OrbitalSeparation::VC(_) => 0,
            OrbitalSeparation::C(_) => 1,
            OrbitalSeparation::M(_) => 2,
            OrbitalSeparation::W(_) => 3,
            OrbitalSeparation::D(_) => 4,
        };

        let mut ord = ord(self).cmp(&ord(other));
        if let Ordering::Equal = ord {
            ord = self.as_metric().cmp(&other.as_metric())
        }
        ord
    }
}