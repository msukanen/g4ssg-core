//! Asteroid belt related stuff lives here …

use crate::core::{designation::IsNamed, orbit::orbital_element::IsOrbitalElement};
//
pub struct AsteroidBelt {

}

impl IsOrbitalElement for AsteroidBelt {
    
}

impl IsNamed for AsteroidBelt {
    fn designation(&self) -> String {
        todo!("IsNamed<AsteroidBelt>: designation()");
    }
}
