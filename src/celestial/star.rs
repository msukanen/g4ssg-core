use core::f64;
use std::{collections::VecDeque, ops::RangeInclusive};

use astrometrics::{AsCelestialRadii, AsMass, AsSpatialUnit, DefoAble, Mass, SpatialUnit, Temperature};
use dicebag::*;
use mshc::Named;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::UNNAMED;
use crate::{celestial::{GasGiantArrangement, ab::{ABRegion, AsteroidBeltType}, gas_giant::*, orbital::{RawOrbitContent, OrbitContent, random_orbital_spacing_ratio}, terrestrial::Terrestrial}, evo::{CFG_STAR_DATA_MIN_MASS, Luminosity, StellarData, StellarDataChoice}, math::LogInterpolator, unit::{Zone, age::{AgeSpan, StellarPopulation}, metrics::kroupa_imf_icdf}};

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

/// Brown dwarf types in ascending surface temperature order.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum BrownDwarfType {
    Y, T, L, M
} impl BrownDwarfType {
    /// Generate a random brownie (could be a cookie, too).
    pub fn random() -> Self {
        match (0.0..=1.0).random_of() {
            x if x < 0.96  => Self::M,
            x if x < 0.975 => Self::L,
            x if x < 0.985 => Self::T,
            _ => Self::Y
        }
    }

    /// Get typical surface temperature range.
    pub fn k_range(&self) -> RangeInclusive<Temperature> {
        match self {
            Self::M => 2_100.0.into()..=3_500.0.into(),
            Self::L => 1_300.0.into()..=2_100.0.into(),
            Self::T => 600.0.into()..=1_1300.0.into(),
            Self::Y => 250.0.into()..=600.0.into(),// https://www.universetoday.com/articles/brrr-jwst-looks-at-the-coldest-brown-dwarf
        }
    }

    /// Get median surface temperature.
    pub fn median_k(&self) -> Temperature {
        let r = self.k_range();
        (r.start() + r.end()) / 2.0
    }

    /// Get random luminosity by type.
    pub fn random_lum(&self) -> f64 {
        match self {
            Self::M => (1e-3..=1e-1).random_of(),
            Self::L => (1e-4..=1e-3).random_of(),
            Self::T => (1e-5..=1e-4).random_of(),
            Self::Y => 1e-6,//…or less.
        }
    }
}

impl TryFrom<Temperature> for BrownDwarfType {
    type Error = String;
    fn try_from(value: Temperature) -> Result<Self, Self::Error> {
        [Self::M, Self::L, Self::T, Self::Y]
            .iter().find(|bdt| bdt.k_range().contains(&value))
            .cloned()
            .ok_or_else(|| {
                if value < *Self::Y.k_range().start() {
                    format!("Too cold - {value} is colder than any known BD")
                } else {
                    format!("Too hot - {value} is hotter than any known BD")
                }
            })
    }
}

/// Star's life stage — from main-sequence to giant(s).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum StarLifeStage {
    /// Neutron/pulsar.
    N,
    /// White Dorf.
    D,
    /// Brownie.
    B(BrownDwarfType),
    /// Main-sequence.
    M,
    /// Main-sequence for giants.
    MG,
    /// Subgiant.
    S,
    /// Wolf-Rayet.
    WR,
    G(GiantStarCategory),
    /// Supergiant.
    SG,
    /// Black hooligan.
    X,
    SMBH
}

#[derive(Debug, Deserialize, Serialize, Clone, Named)]
pub struct Star {
    name: String,
    mass: Mass,
    /// Surface temperature in Kelvin.
    k: Temperature,
    /// Luminosity relative to Sol.
    lum: f64,
    stage: StarLifeStage,
    pop: StellarPopulation,
    /// Radius in AU. Usually™ near neglible except for giants and SMBHs.
    rad: SpatialUnit,
    solid_zone: Zone,
    snow_line: SpatialUnit,
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
} impl Default for StarGenCtx {
    fn default() -> Self {
        Self { smallest_mass: f64::MAX }
    }
}

const G: f64 = 6.67430e-11;
const C: f64 = 2.9979258e8;
// Schwarzschild rad: 2GM/c^2
const TWO_G_OVER_C2: f64 = 2.0 * G / (C*C);

impl Star {
    /// Generate a star in random.
    /// 
    /// # Args
    /// * `name`— obvious…
    /// * `age`— host [star system's][StarSystem] age/population.
    /// * `fz`— forbidden zone.
    /// * `limits`— upper limits for the generated star.
    pub fn random(name: &str, age: &StellarPopulation, fz: &Zone, limits: &mut StarGenCtx) -> Self {
        // Initial (probabilistic random) mass (refined later by current life stage).
        let mass = loop {
            #[cfg(test)]// FYI: test treats `name` as a specific mass representation except when it's specifically [UNNAMED].
            let kr = |x:&str| if x == UNNAMED {kroupa_imf_icdf()} else {x.parse::<f64>().unwrap()};
            #[cfg(not(test))]// …but in production, `name` is irrelevant for real mass generation.
            let kr = |_| kroupa_imf_icdf();
            let mass = kr(name);

            const MIN_MASS_TOLERANCE: f64 = 0.0075;
            if mass <= limits.smallest_mass || (mass - *CFG_STAR_DATA_MIN_MASS).abs() < MIN_MASS_TOLERANCE {
                limits.smallest_mass = mass;
                break mass;
            }
        }.mo();

        // Initial MS surface [Temperature], [Luminosity] and stable/main [AgeSpan].
        let (k, orig_lum, span) = match StellarData::get(mass.into()) {
            StellarDataChoice::Exact(x) => (x.k, x.lum.clone(), x.span.clone()),
            StellarDataChoice::Interpolate(a, b) => (
                mass.as_f64().loginterpol(a.mass.into(), a.k, b.mass.into(), b.k),
                mass.as_f64().loginterpol(a.mass.into(), a.lum.min(), b.mass, b.lum.min()).into(),
                mass.as_f64().loginterpol(a.mass.into(), a.span.mspan().unwrap(), b.mass.into(), b.span.mspan().unwrap()).into(),
            )
        };

        // Figure out current state of affairs for our star-in-making…
        let stage = match &span {
            AgeSpan::Infinite => {
                if mass >= 0.08.mo() { StarLifeStage::M }
                else if mass >= 0.07.mo() { StarLifeStage::B(BrownDwarfType::M)}
                else if mass >= 0.05.mo() { StarLifeStage::B(BrownDwarfType::L)}
                else if mass >= 0.03.mo() { StarLifeStage::B(BrownDwarfType::T)}
                else                      { StarLifeStage::B(BrownDwarfType::Y)}
            }

            AgeSpan::MSpanOnly(s) => {
                let ms_over = age > s;
                let is_massive = mass >= 3.0.mo();
                let frac = age / s;
                let go_neutron =|m|m >= 8.0.mo(); // <8.0M☉ go white dwarf way, heavier shrink to neutron
                let is_midlife =|x|x <= 0.5; // check if `x` is in first half of MS span.

                match (ms_over, is_massive, frac <= 0.05) {
                    // Ded!
                    (true, ..) if mass >= 25.mo()     => StarLifeStage::X,
                    (true, ..) if go_neutron(mass) => StarLifeStage::N,
                    (true, ..)                     => StarLifeStage::D,

                    // Still breathing…
                    (false, false, ..) if mass <= 0.08.mo() => StarLifeStage::B(BrownDwarfType::random()),
                    (false, false, ..) if mass < 3.mo() => StarLifeStage::M, // ye average hippie
                    (false, .., true)                => StarLifeStage::MG, // massive early frac (giant MS "blink")
                    (false, ..) if mass >= 25.mo() && is_midlife(frac) => StarLifeStage::SG, // mid-life superstar
                    (false, ..)                      => StarLifeStage::WR, // eldery bugger, Wolf-Rayeting about
                }
            }
            
            AgeSpan::MSGSpan(m, s, g) => {
                if *m >= age { StarLifeStage::M }
                else if *m+*s >= age { StarLifeStage::S }
                else if *m+*s+*g >= age { StarLifeStage::G(GiantStarCategory::III) }
                else { StarLifeStage::D }
            }
        };

        // Mass refining by current stage.
        //
        // <2.0M☉ stars that fall into D lose major portion of their mass. Let's reflect that too.
        let (orig_mass, mass) = match &stage {
            StarLifeStage::D => if mass <= 0.5 {(mass, mass * 0.95)}
                                else if mass <= 2.0 {(mass, mass * 0.7)}
                                else {(mass, mass.clamp(0.7.mo(), 1.44.mo()))
            }
            
            StarLifeStage::B(bdt) => {
                let refined = match bdt {
                    BrownDwarfType::M => mass.clamp(0.07.mo(), 0.09.mo()),
                    BrownDwarfType::L => mass.clamp(0.03.mo(), 0.07.mo()),
                    BrownDwarfType::T => mass.clamp(0.01.mo(), 0.05.mo()),
                    BrownDwarfType::Y => mass.clamp(0.005.mo(), 0.03.mo()),
                };
                (mass, refined)
            }

            StarLifeStage::N => (mass, (mass * 0.1_f64).clamp(1.1.mo(), 2.0.mo())),

            StarLifeStage::X => (mass, (mass * 0.2_f64).clamp(3.0.mo(), 30.0.mo())),

            StarLifeStage::MG => (mass, mass * 0.95),

            StarLifeStage::SG => (mass, mass * 0.7),

            StarLifeStage::WR => (mass, mass * 0.4),

            _ => (mass, mass)
        };

        // Surface K by current stage.
        let k: Temperature = match &stage {
                StarLifeStage::M => k.jitter_within(100.0).into(),
                StarLifeStage::S => {
                    let msk = k.jitter_within(100.0);
                    let a = age.gyr() - span.mspan().unwrap();
                    //TODO: loginterpol over range?
                    (msk - ((a / span.sspan().unwrap())) * (msk - 4_800.0))
                        .into()
                },
                StarLifeStage::G(_) => (3_000.0_f64..=5_000.0).random_of().into(),
                StarLifeStage::MG => (k.jitter_within(100.0) * 1.1).into(),
                StarLifeStage::SG => (k.jitter_within(100.0) * 0.5).into(),
                StarLifeStage::WR => (k.jitter_within(100.0) * 3.0).into(),
                StarLifeStage::N => Temperature::N,
                StarLifeStage::X |
                StarLifeStage::SMBH => Temperature::X,
                StarLifeStage::D => Temperature::D,
                StarLifeStage::B(x) => x.median_k(),
        };

        // Current luminosity.
        let lum = match &stage {
            StarLifeStage::B(x) => x.random_lum(),
            StarLifeStage::D |
            StarLifeStage::M => match orig_lum {
                Luminosity::LMinOnly(m) => m,
                // all stars which have l-min & l-max should have a useful m-span — but if not… fix the data source.
                Luminosity::LMinMax(a, b) => a + (age / span.mspan().unwrap()).gyr() * (b - a)
            },
            StarLifeStage::S => orig_lum.max(),
            StarLifeStage::G(_) => 25.0 * orig_lum.max(),
            StarLifeStage::MG => orig_lum.max() * 1.2, //TODO: needs refining?
            StarLifeStage::SG => orig_lum.max() * 2.0, //TODO: needs refining?
            StarLifeStage::WR => orig_lum.max() * 3.0, //TODO: needs refining?
            StarLifeStage::N |
            StarLifeStage::X |
            StarLifeStage::SMBH => f64::NAN,
        }.jitter_percentage(10.0);

        let rad = {
            match &stage {
                StarLifeStage::D => (0.008_f32..=0.02).random_of().ro(),
                StarLifeStage::N => 0.000016_f32.ro(),
                StarLifeStage::X |
                StarLifeStage::SMBH => (TWO_G_OVER_C2 * mass).raw().ro(),
                StarLifeStage::M => (155_000.0 * lum.sqrt() / k.sq().as_f64()).ro(),
                _ => SpatialUnit::RO({
                    const T_SUN: f64 = 5772.0;
                    ((lum / (k.as_f64() / T_SUN)).powi(4)).sqrt()
                    })
            }
        };

        // Figure out where planetary solids can exist, excluding Kuiper & Oort stuff.
        let solid_zone = Zone::from(
                // inner limit
                ((0.1 * orig_mass.raw()).max(0.01 * orig_lum.min().sqrt()).au(),
                // outer limit
                (40.0 * orig_mass.raw()).au())
            );
        // Potential placement for the 1st GG, if any.
        let snow_line = (4.85 * orig_lum.min().sqrt()).au();
        
        //
        // Walk the walk, the orbit walk…
        //
        let gga = GasGiantArrangement::random();
        let (fst, fst_is_gg) = if let Some(gga) = &gga {
            (gga.random_distance(&snow_line, &solid_zone), true)
        } else {
            (solid_zone.outer() / (0.05 * 1.d6() as f64 + 1.0), false)
        };
        let mut raw_orbits = VecDeque::new();
        // …walking inwards…
        let mut curr_orbit = fst;
        raw_orbits.push_back(((curr_orbit, ABRegion::Mid), if fst_is_gg {Some(RawOrbitContent::GG)} else {None}));
        loop {
            curr_orbit /= random_orbital_spacing_ratio();
            if curr_orbit < *solid_zone.inner() || fz.contains(&curr_orbit) {
                break;
            }
            raw_orbits.push_front(((curr_orbit, ABRegion::Inner), None));
        }
        // …and then outwards…
        curr_orbit = fst;
        loop {
            curr_orbit *= random_orbital_spacing_ratio();
            if curr_orbit > *solid_zone.outer() || fz.contains(&curr_orbit) {
                break;
            }
            raw_orbits.push_back(((curr_orbit, ABRegion::Outer), None));
        }

        // plonk a few GGs in place, if able to.
        if fst_is_gg {
            for o in raw_orbits.iter_mut() {
                match o {
                    ((distance,_), None) => if orbit_can_contain_gg(&gga, distance, &snow_line) {
                        o.1 = Some(RawOrbitContent::GG);
                    }
                    _ => (/* ignore, already occupied */)
                }
            }
        }

        // …and after that dust settles, lets see about the rest…
        for oidx in 0..raw_orbits.len() {
            let prev_is_gg = oidx > 0 && matches!(raw_orbits[oidx-1].1, Some(RawOrbitContent::GG));
            let next_is_gg = oidx + 1 < raw_orbits.len() && matches!(raw_orbits[oidx+1].1, Some(RawOrbitContent::GG));
            let ((distance,_), stuff) = &mut raw_orbits[oidx];

            if stuff.is_none() {
                *stuff = RawOrbitContent::random(prev_is_gg, next_is_gg, distance, fz, &solid_zone).into();
            }
        }

        // Now that we know what sort of stuff goes where… lets put them there.
        let mut orbits = vec![];
        raw_orbits.iter_mut().for_each(|((distance, region), content)|{
            use RawOrbitContent as R;
            use OrbitContent as O;
            orbits.push(match content {
                None |
                Some(R::Empty) => None,
                Some(R::AB) => (*distance, O::AB(AsteroidBeltType::random(*region, None))).into(),
                Some(R::GG) => (*distance, O::GG(GasGiant::random())).into(),
                Some(R::KB) => (*distance, O::KB).into(),
                Some(R::Oort) => (*distance, O::Oort).into(),
                Some(R::T(sz)) => (*distance, O::T(Terrestrial::random_sized(*sz))).into(),
            });
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
