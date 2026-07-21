//! Here be Gas(oline) Giants

use astrometrics::{AsMass, AsSpatialUnit, Cubed, DefoAble, Mass, MetricsInternalType, SpatialUnit};
use dicebag::{DiceExt, FixedNumberVariance};
use mshc::Named;
use serde::{Deserialize, Serialize};

use crate::{UNNAMED, celestial::{SizeCategory, moons::{Moons, RingSystemDetails}, orbital::OrbitEccentricity}, math::massdensity_to_radius, unit::Zone};

/// An enum used in determining gas giant arrangement of any given star's local system.
#[derive(Debug, Clone, Copy)]
pub enum GasGiantArrangement {
    Conventional,
    Eccentric,
    Epistellar
}

impl GasGiantArrangement {
    /// Generate random GG arrangement (or lack of such).
    pub fn random() -> Option<Self> {
        match 3.d6() {
            ..=10 => None,
            ..=12 => Some(Self::Conventional),
            ..=14 => Some(Self::Eccentric),
            _     => Some(Self::Epistellar)
        }
    }

    /// Generate random [SpatialUnit] for this arrangement
    /// based on the given [Star]'s snow-line radius and/or other factors.
    pub fn random_distance(&self, snow_line: &SpatialUnit, limits: &Zone ) -> SpatialUnit {
        random_gg_distance(Some(self), snow_line, limits).unwrap()
    }
}

/// Generate random [distance][SpatialUnit] for the given [arrangement][GasGiantArrangement], if any.
pub(crate) fn random_gg_distance(
    // Gas giant arrangement, if any.
    gga: Option<&GasGiantArrangement>,
    // A [Star]'s snow-line distance.
    snow_line: &SpatialUnit,
    // A [Star]'s inner/outer zone.
    limits: &Zone
) -> Option<SpatialUnit> {
    if gga.is_none() { return None;}
    
    Some(match gga.unwrap() {
        GasGiantArrangement::Conventional => (0.05 * 2.d6() as f64 + 1.0) * snow_line,
        GasGiantArrangement::Eccentric => 0.125 * 1.d6() as f64 * snow_line,
        GasGiantArrangement::Epistellar => 0.1 * 3.d6() as f64 * limits.inner()
    })
}

/// See if given arrangement lets a GG sit at given distance.
pub(crate) fn orbit_can_contain_gg(gga: &Option<GasGiantArrangement>, distance: &SpatialUnit, snow_line: &SpatialUnit) -> bool {
    match gga {
        None => false,
        // Conventional GGA can have GGs only at/beyond snow-line.
        Some(GasGiantArrangement::Conventional) =>
            if distance >= snow_line && 3.d6() < 16 {true} else {false},
        Some(GasGiantArrangement::Eccentric) =>
            if distance < snow_line && 3.d6() < 9 {true}
            else if distance >= snow_line && 3.d6() < 15 {true}
            else {false},
        Some(GasGiantArrangement::Epistellar) =>
            if distance < snow_line && 3.d6() < 7 {true}
            else if distance >= snow_line && 3.d6() < 15 {true}
            else {false}
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Named)]
pub struct GasGiant {
    name: String,
    size: SizeCategory,
    moons: Moons,
    mass: Mass,
    density: f64,
    radius: SpatialUnit,
    g: MetricsInternalType,
    orbital_period: MetricsInternalType,
    ecc: OrbitEccentricity,
}

impl GasGiant {
    pub(crate) fn random_raw_massdensity(size: SizeCategory) -> (f64, f64) {
        use SizeCategory as C;
        #[inline(always)]
        const fn sm_base_to_massdensity(b:f64,m:f64,d:f64) -> (f64, f64) { (m, d / (m / b)) }
        const SMD: [(f64,f64);2] = [(20.0, 0.22), (30.0, 0.19)];
        const MMD: [f64;9] = [0.18, 0.19, 0.2, 0.22, 0.24, 0.25, 0.26, 0.27, 0.29];
        const LMD: [f64;9] = [0.31, 0.35, 0.4, 0.6,  0.8,  1.0,  1.2,  1.4,  1.6];

        let r = 3.d6();
        match size {
            C::Small => match r {
                    _ if r <= 8 => sm_base_to_massdensity(10.0, 10.0.jitter_within(2.5), 0.42),
                    _ if r <= 10 => sm_base_to_massdensity(15.0, 15.0.jitter_within(2.5), 0.26),
                    x if r <= 12 => {
                        let idx = (x - 11) as usize;
                        sm_base_to_massdensity(SMD[idx].0, SMD[idx].0.jitter_within(5.0), SMD[idx].1)
                    }
                    
                    x => {
                        let base = 40.0 + ((x - 12) * 10) as f64;
                        let mass = base.jitter_within(5.0);
                        sm_base_to_massdensity(base, mass, 0.17)
                    }
                }

            C::Medium => {
                let idx = match r {
                    ..=8 => 0,
                    9|10 => 1,
                    x   => (x - 9) as usize
                };
                let base = 100.0 + (50.0 * idx as f64);
                let mass = base.jitter_within(25.0);
                (mass, MMD[idx] * mass / base)
            }

            C::Large => {
                let (base, jitter, idx) = match r {
                    ..=8 => (600.0, 100.0, 0),
                    9|10 => (800.0, 100.0, 1),
                    x => ((1000.0 + (500.0 * (x-11) as f64)), 250.0, (x - 9) as usize)
                };
                let mass = match r {
                    11 => base.jitter_within(jitter).max(900.0),
                    _  => base.jitter_within(jitter)
                };
                (mass, LMD[idx] * mass / base)
            }

            C::Tiny |
            C::AsteroidCluster => unreachable!("Tiny and asteroids blocked out")
        }
    }

    /// Generate a random [GasGiant].
    /// 
    /// # Args
    /// - `distance` to parent [Star].
    /// - `parent_mass` in M☉.
    /// - `gg_arrangement` of this particular [GasGiant].
    /// - optional preassigned `size` category.
    /// - `iasl` — inside or adjacent to parent [Star]'s snow line.
    /// 
    pub fn random(
        distance: SpatialUnit,
        parent_mass: Mass,
        gg_arrangement: Option<GasGiantArrangement>,
        size: Option<SizeCategory>,
        iasl: bool
    ) -> Self {
        use SizeCategory as C;
        let size = size.unwrap_or_else(|| match 3.d6() + if iasl {4} else {0} {
            ..=10 => C::Small,
            ..=16 => C::Medium,
            _     => C::Large
        });
        let (mass, density) = Self::random_raw_massdensity(size);
        let radius = massdensity_to_radius(mass.me(), density);
        let moons = Moons::random(true, size, radius, distance);
        let g = (density * radius).raw();
        let orbital_period = (distance.au().cubed() / parent_mass.mo().raw()).raw().sqrt();
        let ecc = OrbitEccentricity::random_ecc(&distance, gg_arrangement, true, iasl);

        Self {
            name: UNNAMED.into(),
            size,
            moons,
            mass: mass.mj(),
            density,
            radius: radius.m(),
            g,
            orbital_period,
            ecc,
        }
    }
}

impl RingSystemDetails for GasGiant {
    #[inline(always)]
    fn ring_system(&self) -> Option<super::moons::RingSystem> {
        self.moons.ring_system()
    }
}
