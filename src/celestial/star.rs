use core::f64;
use std::{collections::VecDeque, ops::RangeInclusive};

use astrometrics::{AsCelestialRadii, AsMass, AsSpatialUnit, Cubed, DefoAble, Mass, SpatialUnit, Temperature};
use dicebag::*;
use mshc::Named;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::UNNAMED;
use crate::{celestial::{GasGiantArrangement, ab::{ABRegion, AsteroidBelt}, gas_giant::*, orbital::{OrbitContent, RawOrbitContent, random_orbital_spacing_ratio}, terrestrial::Terrestrial}, evo::{CFG_STAR_DATA_MIN_MASS, Luminosity, StellarData, StellarDataChoice}, math::LogInterpolator, unit::{Zone, age::{AgeSpan, StellarPopulation}, metrics::kroupa_imf_icdf}};

/// Giant star size categories from the smallest to the largest.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd)]
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
    pub fn random_lum(&self, age: StellarPopulation) -> f64 {
        let lum = match self {
            Self::M => (1e-3..=1e-1).random_of(),
            Self::L => (1e-4..=1e-3).random_of(),
            Self::T => (1e-5..=1e-4).random_of(),
            Self::Y => 1e-6,//…or less.
        };
        let age = age.gyr();
        lum * match age {
            _ if age <= 1.0 => 1.0,
            _ if age <= 2.0 => 0.41,
            _ if age <= 3.0 => 0.24,
            _ if age <= 4.0 => 0.16,
            _ if age <= 5.0 => 0.12,
            _ if age <= 6.0 => 0.097,
            _ if age <= 7.0 => 0.08,
            _ if age <= 8.0 => 0.067,
            _ if age <= 9.0 => 0.057,
            _ if age <= 10.0 => 0.05,
            _ if age <= 11.0 => 0.044,
            _ if age <= 12.0 => 0.04,
            _ if age <= 13.0 => 0.036,
            _ if age <= 14.0 => 0.032,
            _ => {
                let lum = 0.03 / (1.0 + (age - 14.0).ln());
                log::info!("BD luminosity multiplier for 14+ Gyr is … a bit of logarithmus at {lum}");
                lum
            }
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
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
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
    orbits: Vec<(SpatialUnit, OrbitContent)>,
    flare_star: bool,
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
    /// Generate a suitable default for [StarGenCtx].
    fn default() -> Self {
        // by default "infinite" is the one and only "larger" value for smallest mass.
        Self { smallest_mass: f64::MAX }
    }
}

const G: f64 = 6.67430e-11;
const C: f64 = 2.9979258e8;
// Schwarzschild rad: 2GM/c^2
const TWO_G_OVER_C2: f64 = 2.0 * G / (C*C);// or TG-C2 for Star Wars fans…

impl Star {
    /// Generate a star in random (to some degree).
    /// 
    /// # Args
    /// - `name`— obvious…
    /// - `age`— host [star system's][StarSystem] age/population.
    /// - `fz`— forbidden zone.
    /// - `limits`— upper limits for the generated star.
    /// 
    /// The [Star] may or may not alter `limits`.
    /// 
    pub fn random(name: &str, age: &StellarPopulation, fz: &Zone, limits: &mut StarGenCtx) -> Self {
        // Initial (probabilistic random) mass (refined later by current life stage).
        let (mass, _is_bd) = loop {
            #[cfg(test)]// FYI: test treats `name` as a specific mass representation except when it's specifically [UNNAMED].
            let kr = |x:&str| if x == UNNAMED {kroupa_imf_icdf()} else {x.parse::<f64>().unwrap()};
            #[cfg(not(test))]// …but in production, `name` is irrelevant for real mass generation.
            let kr = |_| kroupa_imf_icdf();
            let mass = kr(name);

            const MIN_MASS_TOLERANCE: f64 = 0.0075;
            if mass <= limits.smallest_mass || (mass - *CFG_STAR_DATA_MIN_MASS).abs() < MIN_MASS_TOLERANCE {
                limits.smallest_mass = mass;
                // randomize B candidate mass
                if mass < 0.08 {
                    break ((match 3.d6() {
                        ..=8 => 0.015,
                        9|10 => 0.02,
                        11|12 => 0.03,
                        13|14 => 0.04,
                        15 => 0.05,
                        16 => 0.06,
                        _  => 0.07
                    } as f64).jitter_within(0.00333).mo(), true)
                }
                break (mass.mo(), false);
            }
        };

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

        let flare_star = match &stage {
            StarLifeStage::M => orig_mass < 0.6.mo() && 3.d6() > 11,
            _ => false
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
            StarLifeStage::B(x) => x.random_lum(*age),
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
                _ => {
                        const T_SUN: f64 = 5772.0;
                        ((lum / (k.as_f64() / T_SUN)).powi(4)).sqrt()
                    }.ro()
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
        let (fst, fst_is_gg) =
            if let Some(gga) = &gga {
                (gga.random_distance(&snow_line, &solid_zone), true)
            } else {
                (solid_zone.outer() / (0.05 * 1.d6() as f64 + 1.0), false)
            };
        let mut raw_orbits: VecDeque<((SpatialUnit, ABRegion), Option<RawOrbitContent>)> = VecDeque::new();
        // …walking inwards…
        let mut curr_orbit = fst;
        raw_orbits.push_back(((curr_orbit, ABRegion::Mid), if fst_is_gg {Some(RawOrbitContent::GG(gga))} else {None}));
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
                        // mark the 1st GG with GGA; the others get None.
                        o.1 = Some(RawOrbitContent::GG(if fst == *distance { gga } else { None }));
                    }
                    _ => (/* ignore, already occupied */)
                }
            }
        }

        // …and after that dust settles, lets see about the rest…
        for oidx in 0..raw_orbits.len() {
            let prev_is_gg = oidx > 0 && matches!(raw_orbits[oidx-1].1, Some(RawOrbitContent::GG(_)));
            let next_is_gg = oidx + 1 < raw_orbits.len() && matches!(raw_orbits[oidx+1].1, Some(RawOrbitContent::GG(_)));
            let ((distance,_), maybe_content) = &mut raw_orbits[oidx];

            if maybe_content.is_none() {
                *maybe_content = RawOrbitContent::random(prev_is_gg, next_is_gg, distance, fz, &solid_zone).into();
            }
        }

        // Now that we know what sort of stuff goes where… lets put them there.
        let mut orbits = vec![];
        raw_orbits.iter().enumerate().for_each(|(idx, ((distance, region), content))|{
            use RawOrbitContent as R;
            use OrbitContent as O;
            match content {
                Some(R::AB) => orbits.push((*distance, O::AB(AsteroidBelt::random(*region, None)))),
                Some(R::GG(arr)) => orbits.push((*distance, O::GG(
                        GasGiant::random(
                            *distance,
                            *&mass,
                            *arr,
                            None,
                            *distance < snow_line || (idx > 0 && raw_orbits[idx-1].0.0 < snow_line)
                        )))),
                Some(R::KB) => orbits.push((*distance, O::KB)),
                Some(R::Oort) => orbits.push((*distance, O::Oort)),
                Some(R::T(sz)) => orbits.push((*distance, O::T(Terrestrial::random_sized(*age, *distance, *&mass, *sz)))),
                _ => ()
            };
        });
        
        // "That's all folks!", with Looney Tunes…
        Self {
            name: name.into(),
            mass,
            k,
            lum,
            stage,
            pop: *age,
            rad,
            solid_zone, snow_line,
            forbidden_zone: fz.clone(),
            orbits,
            flare_star,
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

    pub fn stage(&self) -> StarLifeStage { self.stage }
    pub fn mass(&self) -> Mass { self.mass }

    // Count terrestrials…
    pub fn num_terrestrials(&self) -> usize {
        let mut c = 0;
        for (_, o) in &self.orbits {
            if matches!(*o, OrbitContent::T(_)) {
                c += 1;
            }
        }
        c
    }

    /// Get the orbital content.
    /// 
    /// Content isn't in any particular guaranteed order — filter as needed.
    /// 
    pub fn orbits(&self) -> &Vec<(SpatialUnit, OrbitContent)> {
        &self.orbits
    }

    /// Calculate the [Star]'s tidal force vs any given planet at any given distance.
    #[inline]
    pub fn tidal_force(&self, distance: SpatialUnit, p_radius: SpatialUnit ) -> f64 {
        Self::tidal_force_m(self.mass, distance, p_radius)
    }

    /// Calculate a (presumed) [Star]'s tidal force vs any given planet at any given distance.
    #[inline]
    pub fn tidal_force_m(mass: Mass, distance: SpatialUnit, p_radius: SpatialUnit ) -> f64 {
        // someone will eventually mistakenly use/forget to convert some e.g. M♃ mass…
        debug_assert!(matches!(mass, Mass::MO(_)), "Using tidal_force_m() with other but M☉ mass is not really optimal… You sure you're dealing with a star?");

        (0.46 * mass.mo().raw() * p_radius.re().raw()) / distance.au().cubed().raw()
    }

    pub fn net_tidal_effect(&self, distance: SpatialUnit, p_radius: SpatialUnit, p_mass: Mass) -> f64 {
        self.pop.gyr() * self.tidal_force(distance, p_radius) / p_mass.me().raw()
    }
}

#[cfg(test)]
mod star_tests {
    #[test]// we need to do this test locally as we need exact mass values (we use star's "name" for that) instead of `kroupa_imf_icdf()`-generated.
    fn edge_case_and_boundary_values() {
        use crate::{celestial::star::{Star, StarGenCtx, StarLifeStage}, unit::{Zone, age::StellarPopulation}};
        const AGE_8KYR: StellarPopulation = StellarPopulation::I2(8.0 / 1_000_000.0);
        const AGE_1MYR: StellarPopulation = StellarPopulation::I2(0.001);
        _ = env_logger::try_init();
        // to make [0.01,100.0] range simpler to step-by-step at 0.01 interval, use [1,10000] i32 insted and divide…
        for x in 1..=10_000 {
            let mut limits = StarGenCtx::default();
            let m = x as f64 / 100.0;
            let star = Star::random(format!("{m:.2}").as_str(), &AGE_8KYR, &Zone::FREE, &mut limits);
            if m < 0.08 {
                assert!(matches!(star.stage(), StarLifeStage::B(_)), "#{} Not B?! {:?}", x, star);
            } else if m < 3.0 {
                assert!(matches!(star.stage(), StarLifeStage::M), "Not M?! {:?}", star);
            } else {
                assert!(matches!(star.stage(), StarLifeStage::MG | StarLifeStage::SG | StarLifeStage::WR), "Not MG|SG|WR?! {:?}", star);
            }

            let star = Star::random(format!("{m:.2}").as_str(), &AGE_1MYR, &Zone::FREE, &mut limits);
            if m < (25.0 - f64::EPSILON) && matches!(star.stage(), StarLifeStage::X) {
                panic!("<25M☉ star ({}) should not result in a black hole!", star.mass())
            }
        }
    }
}
