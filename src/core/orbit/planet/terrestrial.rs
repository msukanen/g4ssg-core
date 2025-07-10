//! Terrestrial planet stuff lives here …

use crate::core::{designation::IsNamed, orbit::orbital_element::IsOrbitalElement};
//
pub struct Terrestrial {

}

impl IsOrbitalElement for Terrestrial {
    
}

impl IsNamed for Terrestrial {
    fn designation(&self) -> String {
        todo!("IsNamed<Terrestrial>: designation()")
    }
}
