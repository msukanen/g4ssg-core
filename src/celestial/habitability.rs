//! Habitability of places.

use astrometrics::MetricsInternalType;

use crate::celestial::{Atmosphere, AtmosphereCondition, AtmosphericPressure, resources::ResourceValue, terrestrial::{TerrestrialSubType, climate::{Climate, ClimateType}, hydrocover::Hydrocover}};

pub fn determine_habitability_affinity(
    rvm: ResourceValue,
    sub: Option<TerrestrialSubType>,
    atm: Option<Atmosphere>,
    hydrocover: Option<MetricsInternalType>,
    climate: Option<Climate>
) -> i32 {
    use AtmosphericPressure as P;
    let mut aff = 0;

    aff += match P::from(atm.as_ref()) {
        P::None | P::Trace => 0,
        _ if atm.is_corrosive() => -2,
        _ if atm.is_toxic() => -1,
        _ if atm.is_suffocating() => 0,
        P::VeryThin => 1,
        P::Thin => 2,
        P::EarthNormal | P::Dense => 3,
        P::VeryDense | P::Superdense => 1,
    };
    aff += if atm.is_marginal() {0} else {1};
    aff += match hydrocover {
        _ if !sub.has_water() => 0,
        Some(h) if h < 1.0 => 0,
        Some(h) if h <= 59.0 => 1,
        Some(h) if h <= 90.0 => 2,
        Some(h) if h <= 99.0 => 1,
        _ => 0
    };
    if atm.is_breathable() {
        if let Some(c) = climate {
            aff += match c.kind() {
                ClimateType::Frozen | ClimateType::VeryCold | ClimateType::VeryHot | ClimateType::Infernal => 0,
                ClimateType::Cold | ClimateType::Hot => 1,
                ClimateType::Earth | ClimateType::Tropical => 3,
                _ => 2
            };
        }
    }

    aff + rvm as i32
}
