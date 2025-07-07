use std::fmt::Display;

use dicebag::DiceExt;

const BYR_THRESHOLD: f64 = 1_000.0;
const BYR_YR_DELTA: f64 = 1_000_000_000.0;

/// Stellar populations from youngest to oldest.
#[derive(Clone, Copy)]
pub enum StellarPopulation {
    ExtremeI,
    YoungI,
    IntermediateI,
    OldI,
    IntermediateII,
    ExtremeII
}

impl StellarPopulation {
    /// Generate random stellar population.
    pub fn new() -> Self {
        match 3.d6() {
            3 => Self::ExtremeI,
            4|5|6 => Self::YoungI,
            7..=10 => Self::IntermediateI,
            11..=14 => Self::OldI,
            15..=17 => Self::IntermediateII,
            _ => Self::ExtremeII
        }
    }

    /// Get base age in BYr.
    pub fn base_age_in_byr(&self) -> f64 {
        match self {
            Self::ExtremeI => 0.0,
            Self::YoungI => 0.1,
            Self::IntermediateI => 2.0,
            Self::OldI => 5.6,
            Self::IntermediateII => 8.0,
            Self::ExtremeII => 10.0,
        }
    }

    /// Generate random Step-A value for final age calculations.
    pub fn gen_step_a(&self) -> f64 {
        (1.d6() - 1) as f64 *
        match self {
            Self::ExtremeI => 0.0,
            Self::YoungI => 0.3,
            _ => 0.6
        }
    }

    /// Generate random Step-B value for final age calculations.
    pub fn gen_step_b(&self) -> f64 {
        (1.d6() - 1) as f64 *
        match self {
            Self::ExtremeI => 0.0,
            Self::YoungI => 0.05,
            _ => 0.1
        }
    }
}

impl Display for StellarPopulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::ExtremeI => "X-I",
            Self::ExtremeII => "X-II",
            Self::IntermediateI => "I-I",
            Self::IntermediateII => "I-II",
            Self::OldI => "O-I",
            Self::YoungI => "Y-I",
        })
    }
}

/// Container for stellar age in years and corresponding population.
/// 
/// See [StellarPopulation].
#[derive(Clone, Copy)]
pub struct StellarAge {
    population: StellarPopulation,
    pub years: f64,                 // NOTE: Earth-years, not Byr!
}

pub trait ByrExt {
    fn as_byr(&self) -> f64;
}

impl ByrExt for f64 {
    fn as_byr(&self) -> f64 {
        if *self < BYR_THRESHOLD { self * BYR_YR_DELTA } else { *self }
    }
}

impl ByrExt for StellarAge {
    fn as_byr(&self) -> f64 {
        self.years / BYR_YR_DELTA
    }
}

impl StellarAge {
    /// Generate random stellar age.
    pub fn new() -> Self {
        let population = StellarPopulation::new();
        StellarAge {
            // Scale the age base and steps into Earth-years.
            years: BYR_YR_DELTA * (population.base_age_in_byr() + population.gen_step_a() + population.gen_step_b()),
            population
        }
    }
}

impl From<(StellarPopulation, f64)> for StellarAge {
    /// Make [StellarAge] from a tuple of [StellarPopulation] and f64 (age in *years*).
    /// NOTE: age value below [BYR_THRESHOLD] is threated as Byr instead of years.
    /// 
    /// # Arguments
    /// * `value`— A tuple of [StellarPopulation] and age in years.
    fn from(value: (StellarPopulation, f64)) -> Self {
        StellarAge { population: value.0, years: if value.1 > BYR_THRESHOLD {value.1} else {value.1 * BYR_YR_DELTA}}
    }
}

impl Display for StellarAge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {} Byr", self.population, self.years / BYR_YR_DELTA)
    }
}
