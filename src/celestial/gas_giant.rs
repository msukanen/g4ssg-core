//! Here be Gas(oline) Giants

use dicebag::DiceExt;

use crate::unit::{Metric, Zone};

/// An enum used in determining gas giant arrangement of any given star's local system.
pub(crate) enum GasGiantArrangement {
    Conventional,
    Eccentric,
    Epistellar
}

impl GasGiantArrangement {
    /// Generate random GG arrangement (or lack of such).
    pub fn random() -> Option<Self> {
        match 3.d6() {
            ..=10 => None,
            ..=12 => Some(Self::Conventional),
            ..=14 => Some(Self::Eccentric),
            _     => Some(Self::Epistellar)
        }
    }

    /// Generate random [distance][Metric] for this arrangement
    /// based on the given [Star]'s snow-line radius and/or other factors.
    pub fn random_distance(&self, snow_line: &Metric, limits: &Zone ) -> Metric {
        random_gg_distance(Some(self), snow_line, limits).unwrap()
    }
}

/// Generate random [distance][Metric] for the given [arrangement][GasGiantArrangement], if any.
pub(crate) fn random_gg_distance(
    // Gas giant arrangement, if any.
    gga: Option<&GasGiantArrangement>,
    // A [Star]'s snow-line distance.
    snow_line: &Metric,
    // A [Star]'s inner/outer zone.
    limits: &Zone
) -> Option<Metric> {
    if gga.is_none() { return None;}
    
    Some(match gga.unwrap() {
        GasGiantArrangement::Conventional => 0.05 * 2.d6() as f64 + 1.0 * snow_line,
        GasGiantArrangement::Eccentric => 0.125 * 1.d6() as f64 * snow_line,
        GasGiantArrangement::Epistellar => 0.1 * 3.d6() as f64 * limits.inner()
    })
}

/// See if given arrangement lets a GG sit at given distance.
pub(crate) fn orbit_can_contain_gg(gga: &Option<GasGiantArrangement>, distance: &Metric, snow_line: &Metric) -> bool {
    match gga {
        None => false,
        // Conventional GGA can have GGs only at/beyond snow-line.
        Some(GasGiantArrangement::Conventional) =>
            if distance >= snow_line && 3.d6() < 16 {true} else {false},
        Some(GasGiantArrangement::Eccentric) =>
            if distance < snow_line && 3.d6() < 9 {true}
            else if distance >= snow_line && 3.d6() < 15 {true}
            else {false},
        Some(GasGiantArrangement::Epistellar) =>
            if distance < snow_line && 3.d6() < 7 {true}
            else if distance >= snow_line && 3.d6() < 15 {true}
            else {false}
    }
}