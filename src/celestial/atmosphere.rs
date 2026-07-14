//! Atmospheric stuff, head in the clouds!

use dicebag::{DiceExt, FixedNumberVariance};
use serde::{Deserialize, Serialize};

use crate::celestial::{composition::{MarginalComposition, ToxicityLevel}, terrestrial::{SizeCategory, TerrestrialSubType}};

pub mod composition;
pub mod pressure;

/// Generate random atmosphere mass, if any.
fn rnd_atm_mass(sub: Option<TerrestrialSubType>, size: Option<SizeCategory>) -> Option<f64> {
    use SizeCategory as C;
    use TerrestrialSubType as T;
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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Atmosphere {
    atm_mass: Option<f64>,
    suffocating: bool,
    corrosive: bool,
    toxicity: Option<ToxicityLevel>,
    marginal: Option<MarginalComposition>,
} impl Atmosphere {
    pub fn random(sub: Option<TerrestrialSubType>, size: Option<SizeCategory>) -> Option<Self> {
        use TerrestrialSubType as T;
        use SizeCategory as C;
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
        }.into()
    }
}
