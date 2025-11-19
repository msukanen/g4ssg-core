use core::f64;
use std::collections::VecDeque;

use dicebag::{DiceExt, FixedNumberVariance, InclusiveRandomRange, PercentageVariance};
use serde::{Deserialize, Serialize};

use crate::{age::{AsYears, StellarPopulation}, celestial::{GasGiantArrangement, gas_giant::orbit_can_contain_gg, orbital::{RawOrbitContent, random_orbital_spacing_ratio}}, evo::{AgeSpan, CFG_STAR_DATA_MIN_MASS, INTERMEDIATE_STARS, Luminosity, MASSIVE_STARS, StellarData}, unit::{AsMetric, Metric, Temperature, Zone, kroupa_imf_icdf}};

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
    /// Main-sequence for giants.
    MG,
    /// Subgiant.
    S,
    /// Wolf-Rayet
    WR,
    G(GiantStarCategory),
    /// Supergiant
    SG,
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
    solid_zone: Zone,
    snow_line: Metric,
    forbidden_zone: Zone,
}

pub enum WhatCanNextStarBe {
    Common,
    Intermediate,
    Massive
} impl Default for WhatCanNextStarBe {
    fn default() -> Self {
        Self::Massive
    }
}

pub struct StarGenCtx {
    smallest_mass: f64,
    next_can_be: WhatCanNextStarBe,
} impl Default for StarGenCtx {
    fn default() -> Self {
        Self { smallest_mass: f64::MAX, next_can_be: WhatCanNextStarBe::Massive }
    }
}

impl Star {
    /// Generate a star in random.
    /// 
    /// # Args
    /// * `name`— obvious…
    /// * `age`— host [star system's][StarSystem] age/population.
    /// * `fz`— forbidden zone.
    /// * `limits`— upper limits for the generated star.
    pub fn random(name: &str, age: &StellarPopulation, fz: &Zone, limits: &mut StarGenCtx) -> Self {
        let gyr = age.gyr();
        let mass = loop {
            let mass = kroupa_imf_icdf();
            const MIN_MASS_TOLERANCE: f64 = 0.0075;
            if mass <= limits.smallest_mass || (mass - *CFG_STAR_DATA_MIN_MASS).abs() < MIN_MASS_TOLERANCE {
                limits.smallest_mass = mass;
                break mass;
            }
        };

        let (mass, stage, k, lum, rad)
        = //
         // A common/intermediate type of a star to fire up?
        //_
        if !matches!(limits.next_can_be, WhatCanNextStarBe::Massive) {
            let (evo, stage, (orig_mass, mass)) = loop {
                let evo = StellarData::random(1.d100() <= 4);
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
                let (orig_mass, mass) = match &stage {
                    StarLifeStage::D => (evo.mass, (0.9..=1.4).random_of()),
                    _ => { let m = evo.mass.jitter_percentage(2.25); (m,m) }
                };
                if orig_mass <= limits.smallest_mass {
                    break (evo, stage, (orig_mass, mass));
                }
            };
            // D's current mass does -not- override earlier smallest mass - it has been bigger bugger back in the day…
            // Check vs it's ye olde mass instead:
            if orig_mass < limits.smallest_mass {
                limits.smallest_mass = orig_mass;
            }
            
            let k = match &stage {
                StarLifeStage::D => Temperature::D,
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
                StarLifeStage::G(_) => (3_000.0..=5_000.0).random_of().into(),
                _ => unreachable!("Truly massive stars are handled elsewhere…")
            };
            let lum = match &stage {
                StarLifeStage::D |
                StarLifeStage::M => match evo.lum {
                    Luminosity::LMinOnly(m) => m,
                    // all stars which have l-min & l-max should have a useful m-span — but if not… fix the data source.
                    Luminosity::LMinMax(a, b) =>
                        a + (gyr / evo.span.mspan()
                                        .unwrap_or_else(|| panic!("Fix the data - missing M-Span for {evo:?}")))
                        * (b - a)
                },
                StarLifeStage::S => evo.lum.max(),
                StarLifeStage::G(_) => 25.0 * evo.lum.max(),
                _ => unreachable!("Truly massive stars are handled elsewhere…")
            }.jitter_percentage(10.0);
            let rad = match &stage {
                StarLifeStage::D => 0.01.rsun(),
                _ => (155_000.0 * lum.sqrt() / k.sq()).as_f64().into()
            };
            (mass, stage, k, lum, rad)
        }
        //
        // Lets deal with the massive stars now then…
        //
        else {
            let evo = StellarData::random_massive();
            let stage = {
                let ms_over = age.as_years() > evo.span_y;
                let is_25er_at_least = evo.mass >= 25.0;
                let frac = age.as_years() / evo.span_y;

                match (ms_over, is_25er_at_least, frac <= 0.05) {
                    // Ded!
                    (true, true, _) => StarLifeStage::X,
                    (true, ..) => StarLifeStage::N,

                    // Still breathing…
                    (false, false, _) => StarLifeStage::M, // ye average hippie
                    (false, _, true) => StarLifeStage::MG, // massive early frac (giant MS "blink")
                    (false, true, _) if frac <= 0.5 => StarLifeStage::SG, // mid-life superstar
                    (false, true, _) => StarLifeStage::WR, // eldery bugger, Wolf-Rayeting about
                }
            };
            let k: Temperature = match &stage {
                StarLifeStage::M => evo.k,
                StarLifeStage::MG => evo.k * 1.1,
                StarLifeStage::SG => evo.k * 0.5,
                StarLifeStage::WR => evo.k * 3.0,
                StarLifeStage::N => 1.0e6,
                StarLifeStage::X |
                StarLifeStage::SMBH => f64::NAN,
                _ => unreachable!("<3M☉ are handled elsewhere.")
            }.into();
            let lum = match &stage {
                StarLifeStage::M => evo.lum,
                StarLifeStage::MG => evo.lum * 1.2,
                StarLifeStage::SG => evo.lum * 2.0,
                StarLifeStage::WR => evo.lum * 3.0,
                StarLifeStage::N |
                StarLifeStage::X |
                StarLifeStage::SMBH => f64::NAN,
                _ => unreachable!("<3M☉ are handled elsewhere.")
            };

            let rad = {
                match &stage {
                    StarLifeStage::N => 1e-4.rsun(),
                    StarLifeStage::X |
                    StarLifeStage::SMBH => {
                        // Schwarzschild rad
                        let g = 6.67430e-11;
                        let c = 2.9979258e8;
                        2.0 * g * evo.mass / (c * c)
                    }.rsun(),
                    _ => Metric::SolRadii({
                        const T_SUN: f64 = 5772.0;
                        ((lum / (k.as_f64() / T_SUN)).powi(4)).sqrt()
                        })
                }
            };
            if evo.mass < 0.005 + MASSIVE_STARS.first().unwrap().mass {
                limits.next_can_be = WhatCanNextStarBe::Common;
            }
            (evo.mass, stage, k, lum, rad)
        };

        // Figure out where planetary solids can exist, excluding Kuiper & Oort stuff.
        let solid_zone = Zone::from(
                // inner limit
                ((0.1 * mass).max(0.01 * lum.sqrt()).au(),
                // outer limit
                (40.0 * mass).au())
            );
        // Potential placement for the 1st GG, if any.
        let snow_line = (4.85 * lum.sqrt()).au();
        
        //
        // Walk the walk, the orbit walk…
        //
        let gga = GasGiantArrangement::random();
        let (fst, fst_is_gg) = if let Some(gga) = &gga {
            (gga.random_distance(&snow_line, &solid_zone), true)
        } else {
            (solid_zone.outer() / (0.05 * 1.d6() as f64 + 1.0), false)
        };
        let mut orbits = VecDeque::new();
        let mut curr_orbit = fst.clone();
        orbits.push_back((curr_orbit.clone(), if fst_is_gg {Some(RawOrbitContent::GG)} else {None}));
        loop {
            curr_orbit /= random_orbital_spacing_ratio();
            if curr_orbit < *solid_zone.inner() || fz.contains(&curr_orbit) {
                break;
            }
            orbits.push_front((curr_orbit.clone(), None));
        }
        curr_orbit = fst.clone();
        loop {
            curr_orbit *= random_orbital_spacing_ratio();
            if curr_orbit > *solid_zone.outer() || fz.contains(&curr_orbit) {
                break;
            }
            orbits.push_back((curr_orbit.clone(), None));
        }

        // plonk a few GGs in place, if able to.
        if fst_is_gg {
            for o in orbits.iter_mut() {
                match o {
                    (distance, None) => if orbit_can_contain_gg(&gga, distance, &snow_line) {
                        o.1 = Some(RawOrbitContent::GG);
                    }
                    _ => (/* ignore, already occupied */)
                }
            }
        }

        // …and after that dust settles, lets see about the rest…
        for oidx in 0..orbits.len() {
            let prev_is_gg = oidx > 0 && matches!(orbits[oidx-1].1, Some(RawOrbitContent::GG));
            let next_is_gg = oidx + 1 < orbits.len() && matches!(orbits[oidx+1].1, Some(RawOrbitContent::GG));
            let (distance, stuff) = &mut orbits[oidx];

            if stuff.is_none() {
                *stuff = RawOrbitContent::random(prev_is_gg, next_is_gg, distance, fz, &solid_zone).into();
            }
        }

        // TODO: potential adjustments here… maybe…

        // Now that we know what sort of stuff goes where… lets put them there.
        orbits.iter().for_each(|(d, content)|{

        });
        
        // "That's all folks!", with Looney Tunes…
        Self {
            name: name.into(),
            mass,
            k,
            lum,
            stage,
            pop: age.clone(),
            rad,
            solid_zone, snow_line,
            forbidden_zone: fz.clone()
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
