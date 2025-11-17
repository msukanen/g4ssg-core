//! Stellar Evolution Charting.

use std::fs;

use dicebag::{DiceExt, RandomOf};
use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// A threshold value used for picking star(s) within certain range from ["pivot mass"][SD_MASS].
const PIVOT_MASS_THRESHOLD: f64 = 0.0475;
/// Config's serde deserializer default for pivot mass.
fn default_pivot_mass_threshold() -> f64 {PIVOT_MASS_THRESHOLD}
const SALTPETER_SLOPE_ALPHA: f64 = 2.35;

/// Evo specs and config live here…
static EVO_FILE: &'static str = "./data/evo.json";
lazy_static! {
    static ref STELLAR_CFG: StellarData = {
        let cfg: StellarData = serde_jsonc::from_str(
            &fs::read_to_string(EVO_FILE).unwrap_or_else(|e| panic!(
                "Exhibit: The Missing File\n\
                Period: Age of Absent Furnaces\n\
                Description: '{EVO_FILE}' could not be found.\n\
                Curator's Note: Editor misplaced the sacred JSON scrolls. Error: {e:?}"))
            ).unwrap_or_else(|e| panic!(
                "Exhibit: The Broken JSON\n\
                Period: Age of Syntax Chaos\n\
                Description: Failed to parse '{EVO_FILE}'\n\
                Curator's Note: Whoever edited this JSON is a moron. Error: {e:?}"
            ));
        if cfg.common.is_empty() {
            panic!(
                "Exhibit: The Great Void\n\
                Period: Age of Missing Stars\n\
                Description: 'common' stars have gone missing…\n\
                Curator's Note: Chuck at least one star in to start the stellar furnace…");
        }
        if cfg.massive.is_empty() {
            panic!(
                "Exhibit: Minutea Absurdica\n\
                Period: Age of Minutea\n\
                Description: 'massive' stars are having a (non-)existential crisis\n\
                Curator's Note: Come on, the universe hasn't just neat little starlets. There's big boyos too! Fix it.");
        }
        cfg
    };
    static ref COMMON_STARS: &'static Vec<StellarEvolution> = &STELLAR_CFG.common;
    static ref MASSIVE_STARS: Vec<&'static StellarEvolutionMassive> = {
        let mut massives: Vec<&'static StellarEvolutionMassive> = STELLAR_CFG.massive.iter().collect();
        massives.sort_by(|a,b| a.mass.total_cmp(&b.mass));
        massives
    };
    static ref MASSIVE_STARS_MIN_MASS: f64 = MASSIVE_STARS.first().unwrap().mass;
    static ref MASSIVE_STARS_MAX_MASS: f64 = MASSIVE_STARS.last().unwrap().mass;
}

/// Stellar data lives here…
#[derive(Debug, Deserialize, Clone)]
pub struct StellarData {
    #[serde(default = "default_pivot_mass_threshold",
            rename = "pivot-mass-threshold")]
    pivot_mass_threshold: f64,
    common: Vec<StellarEvolution>,
    massive: Vec<StellarEvolutionMassive>,
}

/// Stellar age spans for 'common' category of stars.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum AgeSpan {
    /// hundreds of billions of years to trillions and then some.
    /// Virtually "immortal".
    Infinite,
    /// Just main-sequence span.
    MSpanOnly(f64),
    /// Main, subgiant and giant spans.
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

/// A rough "approximate type". Just to give some at glance idea.
/// Not to be used in system generation itself.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ApproxType {
    Spectra(String, u8)
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

/// Evolution specs (for common stars).
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

/// Evolution specs (for massive stars ≥ 3× Sol).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StellarEvolutionMassive {
    pub mass: f64,
    pub k: f64,
    pub lum: f64,
    pub span_y: f64,
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

        // Look up evolution entry/entries that get caught within "pivot mass" threshold range.
        // In default setup this generally grabs only one entry, but…
        let c: Vec<&'static StellarEvolution> =
        COMMON_STARS.iter()
            .filter(|e|
                e.mass >= m - STELLAR_CFG.pivot_mass_threshold &&
                e.mass <= m + STELLAR_CFG.pivot_mass_threshold)
            .collect();
        // Pick an entry in random…
        c.random_of()
    }

    /// Get/generate random massive's evolution data.
    pub fn random_massive() -> StellarEvolutionMassive {
        let u = rand::rng().random_range(0.0..1.0);
        // inverse transform sampling
        let p = 1.0 - SALTPETER_SLOPE_ALPHA;
        let mut a = MASSIVE_STARS_MIN_MASS.powf(p);
        //#[cfg(test)]{log::info!("min-mass araw) {a:?}")}
        let mut b = MASSIVE_STARS_MAX_MASS.powf(p);
        //#[cfg(test)]{log::info!("max-mass braw) {b:?}")}
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        let mass = (a + (b - a) * u)
            .powf(1.0 / p)
            .clamp(*MASSIVE_STARS_MIN_MASS, *MASSIVE_STARS_MAX_MASS);// "Obey the law!", a.k.a. no sneaky border crossers allowed.
        //#[cfg(test)]{log::info!("mass {mass:?}")}
        let bracket = MASSIVE_STARS
            .windows(2)
            .find(|w| w[0].mass <= mass && mass <= w[1].mass)
            .unwrap_or_else(|| panic!("AAARGH! What gives? No stars around {mass} mass?"));
        let (lower, upper) = (bracket[0], bracket[1]);
        
        fn ipow(actual_mass: f64, low_mass: f64, y1: f64, hi_mass: f64, y2: f64) -> f64 {
            let l_actual = actual_mass.ln();
            let l_low_mass = low_mass.ln();
            let l_hi_mass = hi_mass.ln();
            let ly1 = y1.ln();
            let ly2 = y2.ln();
            let t = (l_actual - l_low_mass) / (l_hi_mass - l_low_mass);
            (ly1 + t * (ly2 - ly1)).exp()
        }

        StellarEvolutionMassive {
            k: ipow(mass, lower.mass, lower.k, upper.mass, upper.k),
            span_y: ipow(mass, lower.mass, lower.span_y, upper.mass, upper.span_y),
            lum: ipow(mass, lower.mass, lower.lum, upper.mass, upper.lum),
            mass
        }
    }
}

#[cfg(test)]
mod evo_tests {
    use core::f64;
    use std::ops::{Range, RangeInclusive};

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

    #[test]
    /// Lets verify that Saltpeter'ish generator keeps the massives in line…
    fn random_massive_evo() {
        let _ = env_logger::try_init();
        const LOOPS: Range<usize> = 0..1_000;
        const EXPECTED_AVG_PER_INNER_LOOP: RangeInclusive<f64> = 7.09..=9.54;
        const EXPECTED_TOTAL_AVG: RangeInclusive<f64> = 8.21..=8.27;
        let mut total_avg_mass = 0.0;
        let mut high_std_dev = 0.0;
        for _ in LOOPS {
            let mut min_mass = f64::MAX;
            let mut max_mass = f64::MIN;
            let mut sum_mass = 0.0;
            let mut sum_sq_mass = 0.0;
            for _ in LOOPS {
                let mass = StellarData::random_massive().mass;
                sum_mass += mass;
                sum_sq_mass += mass*mass;
                if mass < min_mass {
                    min_mass = mass
                }
                if mass > max_mass {
                    max_mass = mass
                }
            }
            let avg_mass = sum_mass / LOOPS.end as f64;
            total_avg_mass += avg_mass;
            let variance = (sum_sq_mass / LOOPS.end as f64) - avg_mass.powi(2);
            let std_dev = variance.sqrt();
            if std_dev > high_std_dev {
                high_std_dev = std_dev;
            }
            log::info!("{} loops; range between {min_mass:.3}..{max_mass:.3} with avg. {avg_mass} ± {std_dev:.3}; ", LOOPS.end);
            // average *should* fall between this range with 1000 of 1000-loops
            assert!(EXPECTED_AVG_PER_INNER_LOOP.contains(&avg_mass));
        }
        total_avg_mass /= LOOPS.end as f64;
        log::info!("Total avg: {total_avg_mass:.3} with max ± {high_std_dev:.3}");
        assert!(EXPECTED_TOTAL_AVG.contains(&total_avg_mass));
    }
}