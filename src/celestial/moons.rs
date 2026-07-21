//! Moons and moonlets.
use astrometrics::{AsSpatialUnit, SpatialUnit};
use dicebag::{DiceExt, InclusiveRandomRange};
use serde::{Deserialize, Serialize};

use crate::celestial::SizeCategory;

/// Ring system visibility, if any.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RingSystem {
    Visible,
    Spectacular,
}

pub trait RingSystemDetails {
    fn ring_system(&self) -> Option<RingSystem>;
}

pub trait RingSystemVisibility {
    fn is_spectacular(&self) -> bool;
    fn is_visible(&self) -> bool;
}

impl RingSystemVisibility for RingSystem {
    #[inline(always)]
    fn is_spectacular(&self) -> bool { matches!(self, Self::Spectacular ) }
    #[inline(always)]
    fn is_visible(&self) -> bool { true }
}

impl RingSystemVisibility for Option<RingSystem> {
    #[inline]
    fn is_spectacular(&self) -> bool {
        match self {
            None => false,
            Some(v) => v.is_spectacular()
        }
    }

    #[inline]
    fn is_visible(&self) -> bool {
        match self {
            None => false,
            Some(v) => v.is_visible()
        }
    }
}

/// Moons …and smaller things.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Moons {
    /// Number of these determine the planet's ring system.
    pub moonlets: Option<Vec<SpatialUnit>>,
    /// Count of any (relatively) "major" moons.
    pub major: Option<Vec<(SizeCategory, SpatialUnit)>>,
    /// Number of fringe moonlets, captured asteroids, etc. debris.
    /// Often in eccentric, highly inclined, or even retrograde orbits.
    /// Seldom stable.
    pub fringe: Option<u8>,
}

impl Moons {
    /// Generate random moons for a planet.
    /// 
    /// # Args
    /// - parent is a `gg`?
    /// - `parent_size`
    /// - `distance` to the primary.
    /// 
    pub fn random(gg: bool, parent_size: SizeCategory, parent_radius: SpatialUnit, distance: SpatialUnit) -> Self {
        let rnd_major = |sz: SizeCategory, gg: bool, r: SpatialUnit, n: u8|
            (0..n)
            .map(|_| {
                    let dist = (if gg { 3.0_f64..=15.0 } else { 5.0..=40.0 }).random_of() * r;
                    (sz.random_stepdown(), dist)
                })
            .collect::<Vec<(SizeCategory, SpatialUnit)>>();

        // 
        // Gas giant parent?
        //
        if gg {
            // determine moonlets first
            let modf: i32 = match distance {
                _ if distance <= 0.1.au() => -10,
                _ if distance <= 0.5.au() => -8,
                _ if distance <= 0.75.au() => -6,
                _ if distance <= 1.5.au() => -3,
                _ => 0 };
            let moonlets = {
                let num = (2.d6() + modf).max(0) as usize;
                if num > 0 {
                    let mut moonlets = Vec::with_capacity(num);
                    for _ in 0..num {
                        moonlets.push((1.0_f64..=2.5).random_of() * parent_radius);
                    }
                    moonlets.into()
                } else { None }
            };
            
            // determine major players
            let modf: i32 = match distance {
                _ if distance <= 0.5.au() => -5,
                _ if distance <= 0.75.au() => -4,
                _ if distance <= 1.5.au() => -1,
                _ => 0
            };
            let major: Option<Vec<(SizeCategory, SpatialUnit)>> = match distance {
                _ if distance <= 0.1.au() => None,
                _ => match 1.d6() + modf {
                    ..=0 => None,
                    x => rnd_major(parent_size, true, parent_radius, x as u8).into()
                }
            };

            // determine the 3rd family, the fringe moonlets and other debris
            let modf: i32 = match distance {
                _ if distance <= 0.75.au() => -5,
                _ if distance <= 1.5.au() => -4,
                _ if distance <= 3.au() => -1,
                _ => 0
            };
            let fringe = match distance {
                _ if distance <= 0.5.au() => None,
                _ => match 1.d6() + modf {
                    ..=0 => None,
                    x => Some(x as u8)
                }
            };

            return Self { moonlets, major, fringe };
        }

        //
        // -- Terrestrial-only --
        //

        // too close?
        if distance <= 0.5.au() { return Self { moonlets: None, major: None, fringe: None };}
        
        let mut modf: i32 = match distance {
            _ if distance <= 0.75.au() => -3,
            _ if distance <= 1.5.au() => -1,
            _ => 0
        };
        modf += match parent_size {
            SizeCategory::Tiny => -2,
            SizeCategory::Small => -1,
            SizeCategory::Large => 1,
            _ => 0
        };
        
        // majors? If so, take them and run.
        let num = 1.d6() - 4 + modf;
        if num > 0 { return Self { moonlets: None, major: rnd_major(parent_size, false, parent_radius, num as u8).into(), fringe: None }}
        
        // moonlets?
        let num = 1.d6() - 2 + modf;
        if num > 0 { return Self {
            moonlets: {
                let mut moonlets = Vec::with_capacity(num as usize);
                for _ in 0..num {
                    moonlets.push((2.0..=12.0).random_of() * parent_radius);
                }
                moonlets.into()
            },
            major: None,
            fringe: None }
        }
        
        Self { moonlets: None, major: None, fringe: None }
    }
}

impl RingSystemDetails for Moons {
    /// Get possible ring system's visibility.
    fn ring_system(&self) -> Option<RingSystem> {
        match &self.moonlets {
            None => None,
            Some(m) => match m.len() {
                ..=5 => None,
                ..=9 => Some(RingSystem::Visible),
                _    => Some(RingSystem::Spectacular)
            }
        }
    }
}

#[cfg(test)]
mod moons_tests {
    use std::ops::Range;
    use astrometrics::{AsCelestialRadii, AsMass, AsSpatialUnit, Mass, SpatialUnit};
    use crate::{celestial::{GasGiant, SizeCategory, moons::{Moons, RingSystemDetails, RingSystemVisibility}}, math::massdensity_to_radius};

    const COUNT: usize = 1_000_000;
    const REPEATS: Range<usize> = 0..COUNT;

    fn mdr(size: SizeCategory) -> (Mass, f64, SpatialUnit) {
        let (m, d) = GasGiant::random_raw_massdensity(size);
        let m = m.me();
        let r = massdensity_to_radius(m, d);
        (m, d, r.re())
    }

    fn reap(fun: &str, size: SizeCategory, distance: SpatialUnit) {
        let (_, _, r) = mdr(size);
        let (mut mo, mut ma, mut f, mut v, mut s) = (0,0,0,0,0);
        for _ in REPEATS {
            let moons = Moons::random(true, SizeCategory::Medium, r, distance);
            mo += moons.moonlets.as_ref().map_or_else(|| 0, |n| n.len());
            ma += moons.major.as_ref().map_or_else(|| 0, |n| n.len());
            f += moons.fringe.map_or_else(|| 0, |n| n as usize);
            // count the visibles etc. for some log flavor
            v += if moons.ring_system().is_visible() {1} else {0};
            s += if moons.ring_system().is_spectacular() {1} else {0};
        }
        _ = env_logger::try_init();
        let vis = match (v,s) {
            (0,0) => "".into(),
            (v,0) => format!("\n* visible ring systems: {v}"),
            (0, s) => format!("\n* spectacular ring systems: {s}"),
            (v, s) => format!("\n* visible {v} ring systems, with {s} of them spectacular.")
        };
        log::debug!("for {fun:?}() we have…\n\
            - moonlets: {mo} in {COUNT}\n\
            - major: {ma} in {COUNT}\n\
            - fringe: {f} in {COUNT}{vis}");
        
        // see that number of things sits in expected ranges:
        match distance {
            _ if distance <= 0.1.au() => {
                    assert!(mo >= 100_000 && mo <= 120_000);
                    assert_eq!(ma, 0);
                    assert_eq!(f, 0);
                }
            _ if distance <= 0.5.au() => {
                    assert!(mo >= 530_000 && mo <= 580_000);
                    assert!(ma >= 150_000 && ma <= 190_000);
                    assert_eq!(f, 0);
                }
            _ if distance <= 0.75.au() => {
                    assert!(mo >= 1_500_000 && mo <= 1_600_000);
                    assert!(ma >= 450_000 && ma <= 550_000);
                    assert!(f >= 140_000 && f <= 180_000);
                }
            _ if distance <= 1.5.au() => {
                    assert!(mo >= 3_800_000 && mo <= 4_200_000);
                    assert!(ma >= 2_100_000 && ma <= 2_550_000);
                    assert!(f >= 495_000 && f <= 650_000);
                }
            _ if distance <= 3.0.au() => {
                    assert!(mo >= 6_850_000 && mo <= 7_150_000);
                    assert!(ma >= 3_400_000 && ma <= 4_000_000);
                    assert!(f >= 1_900_000 && f <= 2_600_000);
                }
            _ => {
                    assert!(mo >= 6_850_000 && mo <= 7_150_000);
                    assert!(ma >= 3_400_000 && ma <= 4_000_000);
                    assert!(f >= 2_400_000, "Not enough fringe debris! Got only {f} instances.");
                }
        }
    }

    #[test]
    fn heavy_suppression() {
        reap("heavy_suppression", SizeCategory::Medium, 0.1.au());
    }

    #[test]
    fn mid_suppression() {
        reap("mid_suppression", SizeCategory::Medium, 0.5.au());
    }

    #[test]
    fn low_suppression() {
        reap("low_suppression", SizeCategory::Medium, 0.75.au());
    }

    #[test]
    fn outer_mid_zone() {
        reap("outer_mid_zone", SizeCategory::Medium, 1.5.au());
    }

    #[test]
    fn outer_zone() {
        reap("outer_zone", SizeCategory::Medium, 3.0.au());
    }

    #[test]
    fn outmost_region() {
        reap("outmost_region", SizeCategory::Medium, 6.666666.au());
    }
}