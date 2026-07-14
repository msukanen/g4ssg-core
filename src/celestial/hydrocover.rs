//! Hydrographic Cover — "hydrocover"
//! 

use dicebag::{DiceExt, InclusiveRandomRange, PercentageVariance};

use crate::celestial::{Atmosphere, terrestrial::{SizeCategory, TerrestrialSubType}};

pub struct Hydrocover;
impl Hydrocover {
    pub fn random(atm: Option<&Atmosphere>, sub: Option<TerrestrialSubType>, size: Option<SizeCategory>) -> Option<f64> {
        use TerrestrialSubType as T;
        use SizeCategory as C;
        _ = atm?;
        let sub = sub?;
        let size = size?;
        let clamp_jitter = |c:i32| -> Option<f64> {
            if c <= 0 {None}
            else { (c as f64).jitter_percentage(5.0).min(100.0).into() }
        };
        match (size, sub) {
            (C::Tiny, _) |
            (C::Small, T::Hadean)  |
            (C::Medium, T::Hadean) |
            (_, T::Chthonian)      => return None,
            (C::Small, T::Ice) => (30.0_f64..=80.0).random_of().into(),
            (_, T::Ammonia) => (10.0 * 2.d6() as f64).jitter_percentage(5.0).min(100.0).into(),
            (_, T::Ice) => clamp_jitter(10 * (2.d6() - 10)),
            (C::Medium, T::Ocean | T::Garden) => (45.0_f64..=105.0).random_of().min(100.0).into(),
            // using =120 for biasing toward 100%
            (C::Large, T::Ocean | T::Garden) => (65.0_f64..=120.0).random_of().min(100.0).into(),
            (_, T::Greenhouse) => clamp_jitter(10 * (2.d6() - 7)),
            _ => None
        }
    }
}
