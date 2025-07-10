//! Gas giant related stuff lives here …
//
use dicebag::DiceExt;
use msuk_scifi::unit::distance::au::Au;

use crate::core::orbit::orbital_element::OrbitalElement;

pub enum GasGiantArrangement {
    Conventional { distance: Au },
    Eccentric { distance: Au },
    Epistellar { distance: Au },
}

impl GasGiantArrangement {
    /// Generate random gas giant arrangement.
    ///
    /// # Arguments
    /// * `(inner, outer, snowline)`— triple of the parent [Star]'s orbital zones.
    /// 
    /// # Returns
    /// **None** if no primary GG, otherwise **Some(**[GasGiantArrangement]**)**.
    pub fn new(orbital_zones: &(Au, Au, Au)) -> Option<Self> {
        match 3.d6() {
            ..=10 => None,
            ..=12 => Some(Self::Conventional { distance: orbital_zones.2 * (0.05 * (2.d6() - 2) as f64 + 1.0) }),
            ..=14 => Some(Self::Eccentric { distance: orbital_zones.2 * 0.125 * 1.d6() }),
            _ => Some(Self::Epistellar { distance: orbital_zones.0 * 0.1 * 3.d6() })
        }
    }
}

/// Gas(oline) Giant specs live here.
//
pub struct GasGiant {

}

impl OrbitalElement for GasGiant {
    
}
