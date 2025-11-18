//! Orbital Stuff
//! 

use std::cmp::Ordering;

use dicebag::DiceExt;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{celestial::terrestrial::SizeCategory, unit::{Metric, Zone}};

pub(crate) const ORBIT_RATIO_MIN: f64 = 1.4;
pub(crate) const ORBIT_RATIO_MAX: f64 = 2.0;

/// Orbital eccentricity.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrbitEccentricity {
    /// Eccentricity, generally \[0 → 0.95\].
    ecc: f64,
    /// Average distance.
    avg: Metric,
} impl OrbitEccentricity {
    /// Generate random orbital eccentricity.
    /// 
    /// # Args
    /// 
    /// * `osep`— separation to the nearest relevant neighbor.
    pub fn random(osep: &OrbitSeparation) -> Self {
        let m = match osep {
            OrbitSeparation::VC(_) => -6,
            OrbitSeparation::C(_) => -4,
            OrbitSeparation::M(_) => -2,
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

/// Orbital separation of celestial objects.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum OrbitSeparation {
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
} impl OrbitSeparation {
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

impl Eq for OrbitSeparation {}
impl Ord for OrbitSeparation {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord =
        |x: &OrbitSeparation| match x {
            OrbitSeparation::VC(_) => 0,
            OrbitSeparation::C(_) => 1,
            OrbitSeparation::M(_) => 2,
            OrbitSeparation::W(_) => 3,
            OrbitSeparation::D(_) => 4,
        };

        let mut ord = ord(self).cmp(&ord(other));
        if let Ordering::Equal = ord {
            ord = self.as_metric().cmp(&other.as_metric())
        }
        ord
    }
}

/// Generate a random orbital spacing ratio.
pub fn random_orbital_spacing_ratio() -> f64 {
    let shape = 3.d6();
    let normalized = (shape - 3) as f64 / (18 - 3) as f64;
    // add a bit of jitter...
    let jitter = rand::rng().random::<f64>() / 15.0;
    let clamped = (normalized + jitter).min(1.0);
    ORBIT_RATIO_MIN + clamped * (ORBIT_RATIO_MAX - ORBIT_RATIO_MIN)
    /* 
    match 3.d6() {
        ..=4  => ORBIT_RATIO_MIN,
        ..=6  => ORBIT_RATIO_MIN + ORBIT_RATIO_SLOT_DELTA,
        ..=8  => ORBIT_RATIO_MIN + ORBIT_RATIO_SLOT_DELTA * 2.0,
        ..=12 => ORBIT_RATIO_MIN + ORBIT_RATIO_SLOT_DELTA * 3.0,
        ..=14 => ORBIT_RATIO_MIN + ORBIT_RATIO_SLOT_DELTA * 4.0,
        ..=16 => ORBIT_RATIO_MIN + ORBIT_RATIO_SLOT_DELTA * 5.0,
        _     => ORBIT_RATIO_MAX
    } */
}

/// What's on the orbit?
pub(crate) enum RawOrbitContent {
    Empty,// to distinguish from "not-yet-defined" None.
    AB,// Asteroid belt or Debris (or both)
    T (SizeCategory),
    GG,// Gas giant of some sort
    KB,// Kuiper belt
    Oort,// Oort cloud
} impl RawOrbitContent {
    /// Generate a random orbit content marker. Gas giants are handled separately elsewhere…
    pub fn random(
        prev_is_gg: bool,
        next_is_gg: bool,
        distance: &Metric,
        fz: &Zone,
        limits: &Zone,
    ) -> Self {
        let m
            = if prev_is_gg {-3} else {0}
            + if next_is_gg {-6} else {0}
            + if orbit_adjacent_to_inner_limit(distance, limits) {-3} else {0}
            + if orbit_adjacent_to_outer_limit(distance, limits) {-3} else {0}
            + if fz.orbit_corridor_intersects(distance) {-6} else {0};
        match 3.d6() + m {
            ..=3  => Self::Empty,
            ..=6  => Self::AB,
            ..=8  => Self::T(SizeCategory::Tiny),
            ..=11 => Self::T(SizeCategory::Small),
            ..=15 => Self::T(SizeCategory::Medium),
            _     => Self::T(SizeCategory::Large)
        }
    }
}

/// Check if the given orbit is the next orbit from the absolute inner limit of its parent celestial.
pub fn orbit_adjacent_to_inner_limit(distance: &Metric, limits: &Zone) -> bool {
    let range = (distance/2.0)..=(distance/1.4);
    range.contains(limits.inner())
}

/// Check if the given orbit is the next orbit before the absolute outer limit of its parent celestial.
pub fn orbit_adjacent_to_outer_limit(distance: &Metric, limits: &Zone) -> bool {
    let range = (distance*1.4)..=(distance*2.0);
    range.contains(limits.outer())
}
