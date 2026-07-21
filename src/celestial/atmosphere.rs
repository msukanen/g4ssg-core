//! Atmospheric stuff, head in the clouds!

use astrometrics::MetricsInternalType;
use dicebag::{DiceExt, FixedNumberVariance};
use serde::{Deserialize, Serialize};

use crate::celestial::{SizeCategory, terrestrial::TerrestrialSubType};

pub mod composition; use composition::*;
pub mod pressure;

/// Generate random atmosphere mass, if any.
fn rnd_atm_mass(sub: Option<TerrestrialSubType>, size: Option<SizeCategory>) -> Option<f64> {
    use SizeCategory as C;
    use TerrestrialSubType as T;
    // objects with no `sub` nor `size` defined have no atmos.
    let sub = sub?;
    let size = size?;
    // filter impossibilities out:
    match size {
        C::Tiny => return None,
        C::Small => if !matches!(sub, T::Ice) { return None },
        C::Medium => if matches!(sub, T::Chthonian|T::Hadean) { return None },
        C::Large => if matches!(sub, T::Chthonian) { return None },
        _ => ()
    }

    (0.1 * 3.d6() as f64).jitter_within(0.05).into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtmosphericPressure {
    None,
    Trace,
    VeryThin,
    Thin,
    EarthNormal,
    Dense,
    VeryDense,
    Superdense
}

impl From<f32> for AtmosphericPressure {
    fn from(value: f32) -> Self {
        if (value - 0.0001).abs() < 0.0 { Self::None }
        else if value < 0.01 { Self::Trace }
        else if value <= 0.5 { Self::VeryThin }
        else if value <= 0.80 { Self::Thin }
        else if value <= 1.2 { Self::EarthNormal }
        else if value <= 1.5 { Self::Dense }
        else if value <= 10.0 { Self::VeryDense }
        else { Self::Superdense }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Atmosphere {
    atm_mass: Option<f64>,
    suffocating: bool,
    corrosive: bool,
    toxicity: Option<ToxicityLevel>,
    marginal: Option<MarginalComposition>,
    pressure: Option<f32>,
} impl Atmosphere {
    /// Generate a random [Atmosphere].
    /// 
    /// **NOTE**
    /// the atmosphere's `pressure` needs to be calibrated later
    /// down the generation pipeline with [self.adjust\(..)](Atmosphere::adjust) as its set initially to `None` and
    /// the final value depends on gravity, etc.
    /// 
    pub fn random(sub: Option<TerrestrialSubType>, size: Option<SizeCategory>) -> Option<Self> {
        use TerrestrialSubType as T;
        use SizeCategory as C;
        
        // objects with no `sub` nor `size` defined have no atmos.
        let atm_mass = rnd_atm_mass(sub, size)?;
        let sub = sub?;
        let size = size?;
        
        let (suffocating, corrosive, toxicity, marginal) = match size {
            C::Small => (true, false, (if 3.d6() < 16 { ToxicityLevel::Mild } else { ToxicityLevel::High }).into(), None),
            C::Medium => match sub {
                T::Ice   |
                T::Ocean => (true, false, if 3.d6() < 13 { None } else { ToxicityLevel::Mild.into()}, None),
                T::Garden => (false, false, None, if 3.d6() < 12 { None } else { MarginalComposition::random().into()}),
                _ => (true, true, ToxicityLevel::Lethal.into(), None)
            },
            // C::Tiny has already been blocked by rnd_atm_mass
            _ => match sub {
                T::Ice   |
                T::Ocean => (true, false, ToxicityLevel::High.into(), None),
                T::Garden => (false, false, None, if 3.d6() < 12 { None } else { MarginalComposition::random().into()}),
                _ => (true, true, ToxicityLevel::Lethal.into(), None),
            }
        };

        Self {
            atm_mass: atm_mass.into(),
            suffocating,
            corrosive,
            toxicity,
            marginal,
            pressure: None,
        }.into()
    }

    /// Get atmos mass.
    #[inline(always)]
    pub fn mass(&self) -> Option<f64> { self.atm_mass }

    #[inline(always)]
    pub fn pressure(&self) -> Option<f32> { self.pressure }

    /// Adjust some values based on…
    /// 
    /// # Args
    /// - `sub` type of the planet(oid),
    /// - `size` category of it,
    /// - and the `g` (gravity) of it.
    /// 
    pub fn adjust(&mut self, sub: TerrestrialSubType, size: SizeCategory, g: MetricsInternalType) {
        use TerrestrialSubType as T;
        use SizeCategory as C;

        self.pressure = match self.atm_mass {
            None => None,
            Some(m) => (((match (size, sub) {
                (C::Small, T::Ice) => 10.0,
                (C::Medium, T::Ammonia|T::Ice|T::Ocean|T::Garden) => 1.0,
                (C::Medium, T::Greenhouse) => 100.0,
                (C::Large, T::Ammonia|T::Ice|T::Ocean|T::Garden) => 5.0,
                (C::Large, T::Greenhouse) => 500.0,

                // effectively no atmo to speak of, even if that much
                (C::AsteroidCluster, _)|
                (C::Tiny, _)|
                (_, T::Hadean) => return,

                // trace atmo at best
                _ => 0.01
            }) * m * g) as f32).into()
        };
    }
}

impl From<Option<&Atmosphere>> for AtmosphericPressure {
    fn from(value: Option<&Atmosphere>) -> Self {
        let Some(atm) = value else { return Self::None; };
        let Some(p) = atm.pressure() else { return Self::None; };
        AtmosphericPressure::from(p)
    }
}

pub trait AtmosphereCondition {
    fn is_suffocating(&self) -> bool;
    fn is_toxic(&self) -> bool;
    fn is_corrosive(&self) -> bool;
    fn is_marginal(&self) -> bool;
    #[inline]
    fn is_breathable(&self) -> bool {
        !self.is_suffocating() &&
        !self.is_corrosive() &&
        !self.is_toxic()
    }
}

impl AtmosphereCondition for Atmosphere {
    #[inline(always)]
    fn is_corrosive(&self) -> bool {
        self.corrosive
    }

    #[inline(always)]
    fn is_suffocating(&self) -> bool {
        self.suffocating
    }

    #[inline(always)]
    fn is_toxic(&self) -> bool {
        self.toxicity.is_some()
    }

    #[inline(always)]
    fn is_marginal(&self) -> bool {
        self.marginal.is_some()
    }
}

impl AtmosphereCondition for Option<&Atmosphere> {
    #[inline]
    fn is_corrosive(&self) -> bool {
        match self {
            None => false,
            Some(atm) => atm.is_corrosive()
        }
    }

    #[inline]
    fn is_suffocating(&self) -> bool {
        match self {
            None => false,
            Some(atm) => atm.is_suffocating()
        }
    }

    #[inline]
    fn is_toxic(&self) -> bool {
        match self {
            None => false,
            Some(atm) => atm.is_toxic()
        }
    }

    #[inline]
    fn is_marginal(&self) -> bool {
        match self {
            None => false,
            Some(atm) => atm.is_marginal()
        }
    }

    #[inline]
    fn is_breathable(&self) -> bool {
        match self {
            None => false,
            Some(atm) => atm.is_breathable()
        }
    }
}

impl AtmosphereCondition for Option<Atmosphere> {
    #[inline(always)]
    fn is_corrosive(&self) -> bool {
        self.as_ref().is_corrosive()
    }

    #[inline(always)]
    fn is_suffocating(&self) -> bool {
        self.as_ref().is_suffocating()
    }

    #[inline(always)]
    fn is_toxic(&self) -> bool {
        self.as_ref().is_toxic()
    }

    #[inline(always)]
    fn is_marginal(&self) -> bool {
        self.as_ref().is_marginal()
    }

    #[inline(always)]
    fn is_breathable(&self) -> bool {
        self.as_ref().is_breathable()
    }
}
