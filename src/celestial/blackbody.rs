//! Blackbody Temperature

use astrometrics::Temperature;

use crate::{celestial::{Atmosphere, SizeCategory, terrestrial::TerrestrialSubType}, error::LogicError};

/// Determine blackbody temperature (in Kelvins) for some celestial object.
/// 
/// # Args
/// - `sub` type of a [Terrestrial][crate::celestial::terrestrial::Terrestrial].
/// - `size` category of a [Terrestrial][crate::celestial::terrestrial::Terrestrial].
/// - `atmosphere` of the object.
/// - [`hydrocover`][Hydrocover] of the object.
/// - `k` avg. surface [Temperature].
/// 
/// If `sub` and/or `size` is not given, we treat the object as an *asteroid belt*.
/// 
pub fn determine_blackbody_k(
    sub: Option<TerrestrialSubType>,
    size: Option<SizeCategory>,
    atmosphere: Option<&Atmosphere>,
    hydrocover: Option<f64>,
    k: Temperature
) -> Result<Temperature, LogicError> {
    use TerrestrialSubType as T;
    use SizeCategory as C;

    // if a thing lacks classification, drop it into asteroid/generic bin.
    let (sub, size) = match (sub, size) {
        (Some(s), Some(sz)) => (s,sz),
        _ => return Ok(k / 0.97)
    };
    
    // mostly atmosphereless/static constant ones that don't require atmos mass.
    match (size, sub) {
        (C::Tiny, T::Rock) |
        (_,  T::Chthonian) => return Ok(k / 0.97),
        (C::Tiny, T::Ice)  => return Ok(k / 0.86),

        (C::Small, T::Rock) => return Ok(k / 0.96),
        
        (_, T::Sulfur) => return Ok(k / 0.77),
        (_, T::Hadean) => return Ok(k / 0.67),

        // atmosphere stripped off by a stellar remnant, WR, etc.?
        (C::Medium | C::Large, T::Ice) if atmosphere.is_none() => return Ok(k / 0.86),
        (C::Medium, T::Rock) if atmosphere.is_none() => return Ok(k / 0.96),
        (C::Large,  T::Rock) if atmosphere.is_none() => return Ok(k / 0.93),

        _ => ()
    }

    // simple states being handled above, now some "math" for the rest…
    let atm_mass = atmosphere
        .ok_or(LogicError::AtmosphereMissing)?
        .mass()
        .ok_or(LogicError::AtmosphereMissing)?;
    let hydro = hydrocover.unwrap_or_default();
    let correction_f = |absf,ghf,atm| absf*(1.0+(atm * ghf));
    let (absf, ghf) = match (size, sub) {
        (C::Small, T::Ice) => (0.93, 0.1),
        (_, T::Ammonia)    => (0.84, 0.2),
        (_, T::Ice)        => (0.86, 0.2),
        (_, T::Greenhouse) => (0.77, 2.0),
        _ if hydro <= 20.0 => (0.95, 0.16),
        _ if hydro <= 50.0 => (0.92, 0.16),
        _ if hydro <= 90.0 => (0.88, 0.16),
        _                  => (0.84, 0.16),
    };
    Ok(k / correction_f(absf, ghf, atm_mass))
}
