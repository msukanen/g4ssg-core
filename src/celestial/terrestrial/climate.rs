//! Climate; mainly for [Terrestrial] planet(oids).

use astrometrics::{AsTemperature, Temperature};
use dicebag::{FixedNumberVariance, InclusiveRandomRange};
use serde::{Deserialize, Serialize};

use crate::celestial::{SizeCategory, ab::ABRegion, terrestrial::TerrestrialSubType};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClimateType {
    Frozen,
    VeryCold,
    Cold,
    Chilly,
    Cool,
    Earth,
    Tropical,
    Warm,
    Hot,
    VeryHot,
    Infernal,
}

impl From<Temperature> for ClimateType {
    fn from(temp: Temperature) -> Self {
        let Temperature::K(k) = temp.k() else { panic!("`astrometrics` crate failed to convert temperature into Kelvin?!") };
        if k < 244.0 { Self::Frozen }
        else if k < 255.0 { Self::VeryCold }
        else if k < 266.0 { Self::Cold }
        else if k < 278.0 { Self::Chilly }
        else if k < 289.0 { Self::Cool }
        else if k < 300.0 { Self::Earth }
        else if k < 311.0 { Self::Tropical }
        else if k < 322.0 { Self::Warm }
        else if k < 333.0 { Self::Hot }
        else if k < 344.0 { Self::VeryHot }
        else { Self::Infernal }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Climate {
    kind: ClimateType,
    k: Temperature,
} impl Climate {
    pub fn random(sub: Option<TerrestrialSubType>, size: Option<SizeCategory>, ab_region: ABRegion) -> Self {
        use TerrestrialSubType as T;
        use SizeCategory as C;
        //TODO: region influence on local K

        let range_jitter = |a:f32,b:f32,c:f32| (a..=b).random_of().jitter_within(c).k();

        let k = match (sub, size) {
            (Some(s), Some(sz)) => match (sz, s) {
                (C::Tiny, T::Rock) => range_jitter(140.0, 500.0, 24.0),
                (C::Tiny, _) => range_jitter(80.0, 140.0, 4.0),
                (C::Small, T::Ice) => range_jitter(80.0, 140.0, 4.0),
                (    _, T::Hadean) => range_jitter(50.0, 80.0, 2.0),
                (       _, T::Ice) => range_jitter(80.0, 230.0, 10.0),
                (      _, T::Rock) => range_jitter(140.0, 500.0, 24.0),
                (   _, T::Ammonia) => range_jitter(140.0, 215.0, 5.0),
                (    _, T::Ocean |
                        T::Garden) => range_jitter(250.0, 340.0, 6.0),
                // greenhouse / cthonian
                _ => range_jitter(500.0, 950.0, 30.0)
            },
            _ => (140.0_f32..=500.0).random_of().k()
        };

        Self { kind: k.into(), k }
    }

    pub fn kind(&self) -> ClimateType { self.kind }
    pub fn avg_temperature(&self) -> Temperature { self.k }
}
