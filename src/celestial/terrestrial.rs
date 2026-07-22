//! Terrestrial Planets, Planetoids, Moons, etc.

use astrometrics::{AsMass, AsSpatialUnit, Cubed, DefoAble, Mass, MetricsInternalType, SpatialUnit, Temperature};
use dicebag::DiceExt;
use either::Either;
use mshc::Named;
use serde::{Deserialize, Serialize};

use crate::{UNNAMED, celestial::{Atmosphere, SizeCategory, ab::{ABRegion, AsteroidBelt}, axial::random_axial_tilt, blackbody::determine_blackbody_k, moons::{Moons, RingSystemDetails}, orbital::{OrbitContent, OrbitEccentricity}, rotation::random_rotation_period, star::Star, terrestrial::{geologix::{TectonicActivity, VolcanicActivity}, hydrocover::{Hydrocover, random_hydrocover}}}, unit::age::StellarPopulation};

pub mod climate; use climate::*;
pub mod d_n_g; use d_n_g::*;
pub mod density; use density::*;
pub mod geologix;
pub mod hydrocover;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum TerrestrialOverallType {
    Hostile,
    Barren,
    Garden
} impl TerrestrialOverallType {
    fn random() -> Self {
        match 3.d6() {
            ..=7  => Self::Hostile,
            ..=13 => Self::Barren,
            _     => Self::Garden
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum TerrestrialSubType {
    Ammonia,
    Chthonian,
    Garden,
    Greenhouse,
    Hadean,
    Ice,
    Ocean,
    Rock,
    Sulfur,
}

#[derive(Debug, Clone, Deserialize, Serialize, Named)]
pub struct Terrestrial {
    name: String,
    overall: TerrestrialOverallType,
    sub: TerrestrialSubType,
    size: SizeCategory,
    atm: Option<Atmosphere>,
    hydrocover: Option<MetricsInternalType>,
    climate: Climate,
    blackbody: Temperature,
    core: Core,
    density: f32,
    radius: SpatialUnit,
    g: MetricsInternalType,
    moons: Moons,
    orbital_period: MetricsInternalType,
    ecc: OrbitEccentricity,
    tidal_braking: f64,
    rotation_period: f64,
    axial_tilt: f64,
    volcanism: Option<VolcanicActivity>,
    tectonics: Option<TectonicActivity>,
}

impl Terrestrial {
    /// Generate a random terrestrial planet(oid)… or an asteroid belt if things so judge.
    pub fn random(distance: SpatialUnit, parent_mass: Mass, age: StellarPopulation) -> Either<Self, OrbitContent> {
        use TerrestrialOverallType as O;
        use TerrestrialSubType as S;
        use SizeCategory::*;

        let overall = O::random();
        let (sub, size) = match (3.d6(), &overall) {
            (..=4, O::Hostile) => (S::Chthonian, Medium),
            (..=6, O::Hostile) => (S::Greenhouse, Medium),
            (..=3, O::Barren) => (S::Hadean, Small),
            (   4, O::Barren) => (S::Ice, Small),
            (..=6, O::Barren) => (S::Rock, Small),
            (..=16, O::Garden) => (S::Garden, Medium),
            (..=9, O::Hostile) => (S::Sulfur, Tiny),
            (..=8, O::Barren)  => (S::Rock, Tiny),
            (..=10, O::Barren) => (S::Ice, Tiny),
            (..=12, O::Hostile) => (S::Ammonia, Medium),
            (..=12, O::Barren)  => return Either::Right(OrbitContent::AB(AsteroidBelt::random(ABRegion::Mid, None))),
            (..=14, O::Hostile) => (S::Ammonia, Large),
            (..=14, O::Barren)  => (S::Ocean, Medium),
            (..=16, O::Hostile) => (S::Greenhouse, Large),
            (   15, O::Barren)  => (S::Ice, Medium),
            (   16, O::Barren)  => (S::Hadean, Medium),
            (    _, O::Hostile) => (S::Chthonian, Large),
            (   17, O::Barren) => (S::Ocean, Large),
            (    _, O::Barren) => (S::Ice, Large),
            _ => (S::Garden, Large)
        };

        Either::Left(Self::random_oss(age, distance, parent_mass, overall, sub, size))
    }

    pub fn random_sized(
        age: StellarPopulation,
        distance: SpatialUnit,
        parent_mass: Mass,
        size: SizeCategory
    ) -> Self {
        use TerrestrialOverallType as O;
        use TerrestrialSubType as S;
        use SizeCategory as C;

        let mut overall = TerrestrialOverallType::random();
        while (overall == O::Garden && (size == C::Tiny || size == C::Small)) ||
              (overall == O::Hostile && size == C::Small)
        {
            overall = TerrestrialOverallType::random();
        }
        let sub = match (3.d6(), &overall, size) {
            (_, _, C::AsteroidCluster) |
            (_, O::Garden, C::Tiny)    |
            (_, O::Garden, C::Small)   |
            (_, O::Hostile, C::Small)  => unreachable!(),
            
            (_,     O::Hostile, C::Tiny) => S::Sulfur,
            (..=9,  O::Barren,  C::Tiny) => S::Rock,
            (_,     O::Barren,  C::Tiny) => S::Ice,

            (..=5,  O::Barren,  C::Small) => S::Hadean,
            (..=8,  O::Barren,  C::Small) => S::Ice,
            (_,     O::Barren,  C::Small) => S::Rock,

            (6|7|8, O::Hostile, C::Medium) => S::Greenhouse,
            (..=16, O::Hostile, C::Medium) => S::Ammonia,
            (_,     O::Hostile, C::Medium) => S::Chthonian,

            (..=6,  O::Barren,  C::Medium) => S::Ice,
            (..=8,  O::Barren,  C::Medium) => S::Hadean,
            (_,     O::Barren,  C::Medium) => S::Ocean,

            (..=13, O::Hostile, C::Large) => S::Ammonia,
            (..=16, O::Hostile, C::Large) => S::Greenhouse,
            (_,     O::Hostile, C::Large) => S::Chthonian,

            (..=11, O::Barren,  C::Large) => S::Ocean,
            (_,     O::Barren,  C::Large) => S::Ice,

            (_,     O::Garden,  C::Medium) |
            (_,     O::Garden,  C::Large)  => S::Garden,            
        };

        Self::random_oss(age, distance, parent_mass, overall, sub, size)
    }

    fn random_oss(
        age: StellarPopulation,
        distance: SpatialUnit,
        parent_mass: Mass,
        overall: TerrestrialOverallType,
        sub: TerrestrialSubType,
        size: SizeCategory
    ) -> Self {
        let mut atm = Atmosphere::random(sub.into(), size.into());
        let hydrocover = random_hydrocover(atm.as_ref(), sub.into(), size.into());
        let climate = Climate::random(sub.into(), size.into(), ABRegion::Mid);
        let blackbody = determine_blackbody_k(
                sub.into(),
                size.into(),
                atm.as_ref(),
                hydrocover,
                climate.avg_temperature()
            ).unwrap_or_else(|e| panic!("Could not determine blackbody temperature! {e:?}"));
        let core = Core::select(sub.into(), size.into());
        let density = core.random_density();
        let radius = random_radius(size.into(), blackbody, density);
        let g = random_gravity(density, radius);
        // calibrate the atmos
        if let Some(atm) = &mut atm {
            atm.adjust(sub, size, g);
        }
        let moons = Moons::random(false, size, radius, distance);
        let orbital_period = (distance.au().cubed() / parent_mass.mo().raw()).raw().sqrt();
        let tidal_braking = Star::tidal_force_m(parent_mass, distance, radius) + moons.tidal_force(radius);
        let rotation_period = random_rotation_period(size, false, tidal_braking, orbital_period);
        let axial_tilt = random_axial_tilt(tidal_braking);
        let volcanism = VolcanicActivity::random(age, size, sub.into(), g, Some(&moons), false);
        let tectonics = TectonicActivity::random(volcanism, size, sub.into(), Some(&moons), hydrocover);

        Self {
            name: UNNAMED.into(),
            overall,
            sub,
            size,
            atm,
            hydrocover,
            climate,
            blackbody,
            core,
            density,
            radius,
            g,
            moons,
            orbital_period,
            ecc: OrbitEccentricity::random_ecc(&distance, None, false, false),
            rotation_period,
            tidal_braking,
            axial_tilt,
            volcanism,
            tectonics,
        }
    }
}

impl Hydrocover for TerrestrialSubType {
    /// Return *potential* of liquid water or ice.
    #[inline(always)]
    fn has_water(&self) -> bool {
        match self {
            Self::Ammonia   |
            Self::Chthonian |
            Self::Hadean    |
            Self::Sulfur    => false,
            _ => true,
        }
    }
}

impl Hydrocover for Option<TerrestrialSubType> {
    #[inline]
    fn has_water(&self) -> bool {
        match self {
            None => false,
            Some(h) => h.has_water()
        }
    }
}

impl RingSystemDetails for Terrestrial {
    #[inline(always)]
    fn ring_system(&self) -> Option<super::moons::RingSystem> {
        self.moons.ring_system()
    }
}
