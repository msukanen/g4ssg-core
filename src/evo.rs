//! Stellar Evolution Charting.

use std::fs;

use dicebag::{DiceExt, RandomOf};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// A threshold value used for picking star(s) within certain range from ["pivot mass"][SD_MASS].
static PIVOT_MASS_THRESHOLD: f64 = 0.0475;
/// Config's serde deserializer default for pivot mass.
fn default_pivot_mass_threshold() -> f64 {PIVOT_MASS_THRESHOLD}

/// Evo specs and config live here…
static EVO_FILE: &'static str = "./data/evo.json";
lazy_static! {
    static ref STELLAR_CFG: StellarData = serde_jsonc::from_str(
        &fs::read_to_string(EVO_FILE)
            .unwrap_or_else(|e| panic!("Ok, where'd you put '{EVO_FILE}' this time? Cuz I got {e:?}…"))
    ).expect("JSON error…");

    static ref COMMON_STARS: &'static Vec<StellarEvolution> = &STELLAR_CFG.common;
}

#[derive(Debug, Deserialize, Clone)]
pub struct StellarData {
    #[serde(default = "default_pivot_mass_threshold",
            rename = "pivot-mass-threshold")]
    pivot_mass_threshold: f64,
    common: Vec<StellarEvolution>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum AgeSpan {
    Infinite,
    MSpanOnly(f64),
    MSGSpan(f64, f64, f64)
} impl Default for AgeSpan {
    fn default() -> Self {
        Self::Infinite
    }
} impl AgeSpan {
    pub fn mspan(&self) -> Option<f64> {
        match self {
            Self::Infinite => None,
            Self::MSGSpan(m,..)|
            Self::MSpanOnly(m) => Some(*m)
        }
    }

    pub fn sspan(&self) -> Option<f64> {
        match self {
            Self::MSGSpan(_,s,_) => Some(*s),
            _ => None
        }
    }

    pub fn gspan(&self) -> Option<f64> {
        match self {
            Self::MSGSpan(_,_,g) => Some(*g),
            _ => None
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ApproxType {
    Spectra(String, u8)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Luminosity {
    LMinOnly(f64),
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StellarEvolution {
    pub mass: f64,
    #[serde(rename = "approx-type")]
    approx_type: ApproxType,
    pub k: f64,
    pub lum: Luminosity,
    #[serde(default)]
    pub span: AgeSpan
}

/// "Pivot mass" markers for selecting star(s) in random from [COMMON_STARS].
static SD_MASS: [f64; 34] = [
    2.0,   1.9,  1.8,   1.7,  1.6,   1.5,
    1.45,  1.4,  1.35,  1.3,  1.25,  1.2,
    1.15,  1.1,  1.05,  1.0,  0.95,  0.9,
    0.85,  0.8,  0.75,  0.7,  0.65,  0.6,
    0.55,  0.5,  0.45,  0.4,  0.35,  0.3,
    0.25,  0.2,  0.15,  0.1];

impl StellarData {
    /// Get a random [evolution][StellarEvolution] entry.
    pub fn random() -> &'static StellarEvolution {
        let r = || 3.d6();
        // Choose a "pivot mass" to play around. As per IMF, heavily biased toward low-mass stars.
        let m = SD_MASS[match 3.d6() {
            ..=3 => match r() {..=10 => 0, _=> 1},
            4 => match r() {..=8 => 2, ..=11 => 3, _=> 4},
            5 => match r() {..=7 => 5, ..=10 => 6, ..=12 => 7, _=> 8},
            6 => match r() {..=7 => 9, ..=9 => 10, 10 => 11, ..=12 => 12, _=> 13},
            7 => match r() {..=7 => 14, ..=9 => 15, 10 => 16, ..=12 => 17, _=> 18},
            8 => match r() {..=7 => 19, ..=9 => 20, 10 => 21, ..=12 => 22, _=> 23},
            9 => match r() {..=8 => 24, ..=11 => 25, _=> 26},
            10 => match r() {..=8 => 27, ..=11 => 28, _=> 29},
            11 => 30,
            12 => 31,
            13 => 32,
            _ => 33
        }];

        // Pick a random [StellarEvolution] within a certain range from the pivot mass.
        // In default setup this generally grabs only one entry, but…
        let c: Vec<&'static StellarEvolution> =
        COMMON_STARS.iter()
            .filter(|e| e.mass >= m - 0.0475 && e.mass <= m + 0.0475)
            .collect();
        c.random_of()
    }
}

#[cfg(test)]
mod evo_tests {
    use super::*;

    #[test]
    fn evo_file_integrity() {
        let _ = STELLAR_CFG.clone();
    }

    #[test]
    fn random_evo() {
        let e = StellarData::random();
        let _ = env_logger::try_init();
        log::info!("{e:?}");
    }
}