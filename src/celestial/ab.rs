//! Asteroid Belts and Other Debris

use dicebag::{DiceExt, IsOne};
use serde::{Deserialize, Serialize};

/// Asteroid belt regions.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum ABRegion { Inner, Mid, Outer }

/// Asteroid belt subtypes.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum ABSubtype {
    // C-stuff
    C, Ch, Cgh, B,
    // S's
    S, K, L, V,
    // M/X
    M, E, P, D,
}

/// Asteroid belt types.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum AsteroidBeltType {
    C (ABSubtype),
    M (ABSubtype),
    S (ABSubtype),
} impl AsteroidBeltType {
    pub fn random(region: ABRegion, albedo_hint: Option<f32>) -> Self {
        // Helper for S-kind
        fn ab_s(region: ABRegion) -> AsteroidBeltType {
            let roll = 1.d100();
            AsteroidBeltType::S(match region {
                ABRegion::Inner => match roll {..=75 => ABSubtype::S, ..=90 => ABSubtype::K, ..=97 => ABSubtype::L, _=> ABSubtype::V },
                ABRegion::Mid   => match roll {..=65 => ABSubtype::S, ..=85 => ABSubtype::K, ..=95 => ABSubtype::L, _=> ABSubtype::V },
                ABRegion::Outer => match roll {..=70 => ABSubtype::S, ..=88 => ABSubtype::K, ..=97 => ABSubtype::L, _=> ABSubtype::V },
            })
        }

        // Helper for M's
        fn ab_m(albedo_hint: Option<f32>) -> AsteroidBeltType {
            // for splitting X-group… high→E, very low→P/D, else M
            let a = albedo_hint.unwrap_or(0.08);
            AsteroidBeltType::M(match a {
                _ if a >= 0.3 => ABSubtype::E,
                _ if a <= 0.05 => if 1.d2().is_one() { ABSubtype::P } else { ABSubtype::D },
                _ => match 1.d100() {..=10 => ABSubtype::E, ..=20 => ABSubtype::P, ..=25 => ABSubtype::D, _=> ABSubtype::M }
            })
        }

        let roll = 1.d100();
        match roll {
            // 75% C
            ..=75 => Self::C ({
                let roll = 1.d100();
                match region {
                    ABRegion::Outer => match roll {..=40 => ABSubtype::Ch, ..=55 => ABSubtype::Cgh, ..=80 => ABSubtype::B, _=> ABSubtype::C },
                    ABRegion::Mid   => match roll {..=30 => ABSubtype::Ch, ..=40 => ABSubtype::Cgh, ..=60 => ABSubtype::B, _=> ABSubtype::C },
                    ABRegion::Inner => match roll {..=15 => ABSubtype::Ch, ..=25 => ABSubtype::Cgh, ..=40 => ABSubtype::B, _=> ABSubtype::C }
                }
            }),

            // 15-20% S
            //  5-10% M
            _ => {
                match 1.d(25) {
                    20.. => if 1.d2().is_one() { ab_s(region) } else { ab_m(albedo_hint) },
                    ..=15 => ab_s(region),
                    _ => ab_m(albedo_hint)
                }
            }
        }
    }
}