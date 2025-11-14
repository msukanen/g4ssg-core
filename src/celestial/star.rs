use dicebag::{FixedNumberVariance, PercentageVariance};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{age::StellarAge, evo::{AgeSpan, Luminosity, StellarData, StellarEvolution}};

/// Star's life stage — from main-sequence to giant(s).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StarLifeStage {
    M,
    S,
    G,
    D,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Star {
    pub name: String,
    mass: f64,
    k: f64,
    lum: f64,
    stage: StarLifeStage,
    pop: StellarAge,
}

impl Star {
    pub fn random(name: &str, age: StellarAge) -> Self {
        let evo = StellarData::random();
        let age = age.gyr();
        let stage = match evo.span {
            AgeSpan::Infinite => StarLifeStage::M,
            AgeSpan::MSpanOnly(m) => if m >= age { StarLifeStage::M } else { StarLifeStage::D },
            AgeSpan::MSGSpan(m, s, g) => {
                if m >= age { StarLifeStage::M }
                else if m+s >= age { StarLifeStage::S }
                else if m+s+g >= age { StarLifeStage::G }
                else { StarLifeStage::D }
            }
        };
        let mass = match &stage {
            StarLifeStage::D => rand::rng().random_range(0.9..=1.4),
            _ => evo.mass.upto_delta(0.025)
        };
        let lum = match &stage {
            StarLifeStage::D => 0.000001,
            StarLifeStage::M => match evo.lum {
                Luminosity::LMinOnly(m) => m,
                // all stars which have l-min & l-max should have a useful m-span — but if not… fix the data source.
                Luminosity::LMinMax(a, b) => a + ((age / evo.span.mspan()
                    .unwrap_or_else(|| panic!("Fix the data - missing M-Span for {evo:?}"))) * (b - a))
            },
            StarLifeStage::S => evo.lum.max(),
            StarLifeStage::G => 25.0 * evo.lum.max()
        }.delta(10);
    }
}