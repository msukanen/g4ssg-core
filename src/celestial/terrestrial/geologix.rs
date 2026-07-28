//! Geologic activity.

use astrometrics::MetricsInternalType;
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{celestial::{SizeCategory, moons::Moons, terrestrial::TerrestrialSubType}, unit::age::StellarPopulation};

/// Volcanic activity level.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum VolcanicActivity {
    Light,
    Moderate,
    Heavy,
    Extreme,
} impl VolcanicActivity {
    pub fn random(
        age: StellarPopulation,
        size: SizeCategory,
        sub: Option<TerrestrialSubType>,
        g: MetricsInternalType,
        moons: Option<&Moons>,
        moon_of_gg: bool,
    ) -> Option<Self> {
        let mut modf = (g as f64 / age.gyr() * 40.0).round() as i32;
        if let Some(sub) = sub {
            modf += match (size, sub) {
                (SizeCategory::Tiny, TerrestrialSubType::Sulfur) => 60,
                _ => 0
            };

            if let Some(moons) = moons {
                if let Some(major) = &moons.major {
                    modf += match major.len() {
                        1 => 5,
                        2.. => 10,
                        _ => 0
                    };
                }
            }
        }
        if moon_of_gg {
            modf += 5;
        }
        
        match 3.d6() + modf {
            ..=16 => None,
            ..=20 => Self::Light.into(),
            ..=26 => Self::Moderate.into(),
            ..=70 => Self::Heavy.into(),
            _     => Self::Extreme.into()
        }
    }
}

/// Tectonic activity level.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TectonicActivity {
    Light,
    Moderate,
    Heavy,
    Extreme,
} impl TectonicActivity {
    pub fn random(
        va: Option<VolcanicActivity>,
        size: SizeCategory,
        sub: Option<TerrestrialSubType>,
        moons: Option<&Moons>,
        hydrocover: Option<MetricsInternalType>,
    ) -> Option<Self> {
        if matches!(size, SizeCategory::Tiny|SizeCategory::Small) { return None; }
        let mut modf = match va {
            None => -8,
            Some(va) => match va {
                VolcanicActivity::Light => -4,
                VolcanicActivity::Heavy => 4,
                VolcanicActivity::Extreme => 8,
                _ => 0
            }
        };
        modf += match hydrocover {
            None => -4,
            Some(h) if h < 50.0 => -2,
            _ => 0,
        };
        if let Some(_) = sub {
            if let Some(moons) = moons {
                if let Some(major) = &moons.major {
                    modf += match major.len() {
                        1 => 2,
                        2.. => 4,
                        _ => 0,
                    };
                }
            }
        }

        match 3.d6() + modf {
            ..=6 => None,
            ..=10 => Self::Light.into(),
            ..=14 => Self::Moderate.into(),
            ..=18 => Self::Heavy.into(),
            _     => Self::Extreme.into()
        }
    }
}
