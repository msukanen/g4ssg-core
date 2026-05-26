//! Stellar Evolution Charting.

use std::fs;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::unit::{age::AgeSpan, metrics::kroupa_imf_icdf};

/// A threshold value used for picking star(s) within certain range from ["pivot mass"][SD_MASS].
const PIVOT_MASS_THRESHOLD: f64 = 0.0475;
/// Config's serde deserializer default for pivot mass.
fn default_pivot_mass_threshold() -> f64 {PIVOT_MASS_THRESHOLD}

/// Evo specs and config live here…
static EVO_FILE: &'static str = "./data/evo.json";
lazy_static! {
    static ref STELLAR_CFG: StellarData = {
        let cfg: StellarData = serde_jsonc::from_str(
            &fs::read_to_string(EVO_FILE).unwrap_or_else(|e| panic!(
                "Exhibit: The Missing File\n\
                Period: Age of Absent Furnaces\n\
                Description: '{EVO_FILE}' could not be found.\n\
                Editor misplaced the sacred JSON scrolls. Error: {e:?}"))
            ).unwrap_or_else(|e| panic!(
                "Exhibit: The Broken JSON\n\
                Period: Age of Syntax Chaos\n\
                Description: Failed to parse '{EVO_FILE}'\n\
                Whoever edited this JSON is a moron. Error: {e:?}"
            ));
        if cfg.data.is_empty() {
            panic!(
                "Exhibit: The Great Void\n\
                Period: Age of Missing Stars\n\
                Description: 'common' stars have gone missing…\n\
                Chuck at least one star in to start the stellar furnace…");
        }
        cfg
    };
    pub(crate) static ref STAR_DATA: Vec<&'static StellarEvolution> = {
        let mut sorted: Vec<&'static StellarEvolution> = STELLAR_CFG.data.iter()
            .collect::<Vec<&'static StellarEvolution>>();
        sorted.sort_by(|a,b| a.mass.total_cmp(&b.mass));
        sorted
    };
    pub(crate) static ref COMMON_STARS: Vec<&'static StellarEvolution> = {
        STAR_DATA.iter()
            .copied()
            .filter(|e| e.mass <= 2.0)
            .collect::<Vec<&'static StellarEvolution>>()
    };
    pub(crate) static ref INTERMEDIATE_STARS: Vec<&'static StellarEvolution> = {
        STAR_DATA.iter()
            .copied()
            .filter(|e| e.mass > 2.0 && e.mass < 3.0)
            .collect::<Vec<&'static StellarEvolution>>()
    };
    pub(crate) static ref MASSIVE_STARS: Vec<&'static StellarEvolution> = {
        STAR_DATA.iter()
            .copied()
            .filter(|e| e.mass >= 3.0)
            .collect::<Vec<&'static StellarEvolution>>()
    };
    pub(crate) static ref CFG_STAR_DATA_MIN_MASS: f64 = STAR_DATA.first().unwrap().mass;
    pub(crate) static ref CFG_STAR_DATA_MAX_MASS: f64 = STAR_DATA.last().unwrap().mass;
}

#[inline]
fn is_common(mass: f64) -> bool {
    !is_heavier_than_common(mass)
}

#[inline]
fn is_heavier_than_common(mass: f64) -> bool {
    mass > COMMON_STARS.last().unwrap().mass
}

#[inline]
fn is_intermediate(mass: f64) -> bool {
    !is_massive(mass) && is_heavier_than_common(mass)
}

#[inline]
fn is_massive(mass: f64) -> bool {
    mass >= MASSIVE_STARS.first().unwrap().mass
}

/// Stellar data lives here…
#[derive(Debug, Deserialize, Clone)]
pub struct StellarData {
    #[serde(default = "default_pivot_mass_threshold",
            rename = "pivot-mass-threshold")]
    pivot_mass_threshold: f64,
    data: Vec<StellarEvolution>,
}

/// Initial luminosity at main-sequence phase.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Luminosity {
    /// L-min value only. L-max is not different enough to bother listing separately.
    LMinOnly(f64),
    /// L-min and L-max.
    LMinMax(f64, f64)
} impl Luminosity {
    pub fn min(&self) -> f64 {
        match self {
            Self::LMinMax(m,..) |
            Self::LMinOnly(m)   => *m
        }
    }

    pub fn max(&self) -> f64 {
        match self {
            Self::LMinMax(_,m) |
            // max for l-min-only is the same as l-min
            Self::LMinOnly(m)  => *m
        }
    }
}

impl From<f64> for Luminosity {
    fn from(value: f64) -> Self {
        Self::LMinOnly(value)
    }
}

/// Evolution specs (for common stars).
#[derive(Debug, Deserialize, Clone)]
pub struct StellarEvolution {
    pub mass: f64,
    pub k: f64,
    pub lum: Luminosity,
    #[serde(default)]
    pub span: AgeSpan
}

#[derive(Debug)]
pub(crate) enum StellarDataChoice {
    Exact(&'static StellarEvolution),
    Interpolate(&'static StellarEvolution, &'static StellarEvolution)
}

impl StellarData {
    /// Fetches either more-or-less exact data (when able to) or neighbors from which
    /// exact data has to be interpolated.
    pub fn get(needle_mass: f64) -> StellarDataChoice {
        // Common/Intermediate masses can be "pinpointed" with relative impunity…
        if needle_mass <= 3.0 {
            StellarDataChoice::Exact(STAR_DATA.iter()
                .min_by(|a,b| {
                    (a.mass - needle_mass).abs()
                        .partial_cmp(&(&b.mass - needle_mass).abs())
                        .unwrap()
                })
                .unwrap())
        }
        // …while Massive stars' data has the wider gaps the more massive we're aiming for.
        else {
            STAR_DATA.windows(2)
                .find(|w| w[0].mass <= needle_mass && w[1].mass >= needle_mass)
                .map(|w| StellarDataChoice::Interpolate(&w[0], &w[1]))
                // mass out of recorded cap - resort to last entry
                .unwrap_or_else(|| StellarDataChoice::Exact(STAR_DATA.last().unwrap()))
        }
    }
    
    /// Get a random [evolution][StellarEvolution] entry.
    pub fn random() -> StellarDataChoice {
        // fetch StellarEvolution data by probabilistic random mass.
        Self::get(kroupa_imf_icdf())
    }
}
