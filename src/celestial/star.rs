use std::ops::RangeInclusive;

use dicebag::{DiceExt, FixedNumberVariance, InclusiveRandomRange, PercentageVariance};
use serde::{Deserialize, Serialize};

use crate::{age::StellarPopulation, celestial::orbital::OrbitalEccentricity, evo::{AgeSpan, Luminosity, StellarData}, unit::{AsMetric, Metric, Temperature, Zone}};

/// Giant star size categories from the smallest to the largest.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GiantStarCategory {
    IV,
    III,
    II,
    Ib,
    Iab,
    Ia,
    #[serde(rename = "Ia+")]
    IaP, // Hype giant
}

/// Star's life stage — from main-sequence to giant(s).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StarLifeStage {
    /// Neutron/pulsar
    N,
    /// White Dorf
    D,
    /// Main-sequence.
    M,
    /// Subgiant.
    S,
    /// Wolf-Rayet
    WR,
    G(GiantStarCategory),
    /// Black hooligan
    X,
    SMBH
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Star {
    pub name: String,
    /// Mass in Sol masses.
    mass: f64,
    /// Surface temperature in Kelvin.
    k: Temperature,
    /// Luminosity relative to Sol.
    lum: f64,
    stage: StarLifeStage,
    pop: StellarPopulation,
    /// Radius in AU. Usually™ near neglible except for giants and SMBHs.
    rad: Metric,
    solid_zone: RangeInclusive<Metric>,
    snow_line: Metric,
}

impl Star {
    /// Generate a star in random.
    /// 
    /// # Args
    /// * `name`— obvious…
    /// * `age`— host [star system's][StarSystem] age/population.
    /// * `fz`— forbidden zone.
    pub fn random(name: &str, age: &StellarPopulation, fz: &Zone) -> Self {
        let evo = StellarData::random();
        let gyr = age.gyr();
        let stage = match evo.span {
            AgeSpan::Infinite => StarLifeStage::M,
            AgeSpan::MSpanOnly(m) => if m >= gyr { StarLifeStage::M } else { StarLifeStage::D },
            AgeSpan::MSGSpan(m, s, g) => {
                if m >= gyr { StarLifeStage::M }
                else if m+s >= gyr { StarLifeStage::S }
                else if m+s+g >= gyr { StarLifeStage::G(GiantStarCategory::III) }
                else { StarLifeStage::D }
            }
        };
        let mass = match &stage {
            StarLifeStage::D => (0.9..=1.4).random_of(),
            _ => evo.mass.jitter_percentage(2.25)
        };
        let k = match &stage {
            StarLifeStage::D |
            StarLifeStage::N |
            StarLifeStage::X |
            StarLifeStage::SMBH => Temperature::D,
            StarLifeStage::M => evo.k.jitter_within(100.0).into(),
            StarLifeStage::S => {
                let msk = evo.k.jitter_within(100.0);
                let a = gyr - evo.span.mspan().unwrap_or_else(
                    || panic!("Fix the data — missing M-span for {evo:?}"));
                
                (msk - ((a / evo.span.sspan().unwrap_or_else(
                    || panic!("Fix the data — missing S-span for {evo:?}")
                )) * (msk - 4_800.0)))
                    .into()
            },
            StarLifeStage::WR => unimplemented!("TODO: WR-temperature"),
            StarLifeStage::G(_) => (3_000.0..=5_000.0).random_of().into()
        };
        let lum = match &stage {
            StarLifeStage::SMBH |
            StarLifeStage::X    => 0.0,
            StarLifeStage::D |
            StarLifeStage::N => (0.0001..=0.001).random_of(),
            StarLifeStage::M => match evo.lum {
                Luminosity::LMinOnly(m) => m,
                // all stars which have l-min & l-max should have a useful m-span — but if not… fix the data source.
                Luminosity::LMinMax(a, b) =>
                    a + (gyr / evo.span.mspan()
                                    .unwrap_or_else(|| panic!("Fix the data - missing M-Span for {evo:?}")))
                    * (b - a)
            },
            StarLifeStage::WR => unimplemented!("TODO: WR-luminosity"),
            StarLifeStage::S => evo.lum.max(),
            StarLifeStage::G(_) => 25.0 * evo.lum.max()
        }.jitter_percentage(10.0);
        let rad = match &stage {
            StarLifeStage::D => 0.01.rsun(),
            StarLifeStage::N => 1e4.rsun(),
            StarLifeStage::X |
            StarLifeStage::SMBH => {
                // Schwarzschild rad
                let g = 6.67430e-11;
                let c = 2.9979258e8;
                2.0 * g * mass / (c * c)
            }.rsun(),
            StarLifeStage::WR => unimplemented!("TODO: WR-radius"),
            _ => (155_000.0 * lum.sqrt() / (&k * &k)).as_f64().into()
        };
        
        Self {
            name: name.into(),
            mass,
            k,
            lum,
            stage,
            pop: age.clone(),
            rad,
            solid_zone: (0.1 * mass).max(0.01 * lum.sqrt()).au()..=(40.0 * mass).au(),
            snow_line: (4.85 * lum.sqrt()).au(),
        }
    }

    /// Some dubstep light curving is a-ok, right?
    #[cfg(feature = "local_test")]
    pub fn dubstep(steps: usize, samples_per_step: usize) -> Vec<f64> {
        let mut curve = Vec::with_capacity(steps);
        for t in 0..steps {
            let phase = t as f64 / steps as f64 * std::f64::consts::TAU;
            // Base pulse (Cepheid-style)
            let pulsation = (phase.sin() * 0.5 + 1.0);
            // Wub-wub!
            let wobble = (phase * 4.0).sin() * 0.3;
            // Drop!
            let drop = if t % (steps / 8) == 0 { 2.0 } else { 0.0 };
            
            let amp = pulsation + wobble + drop;
            for _ in 0..samples_per_step {
                curve.push(amp);
            }
        }
        curve
    }
}