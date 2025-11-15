use serde::{Deserialize, Serialize};

use crate::{celestial::{orbital::{OrbitalEccentricity, OrbitalSeparation}, star::Star}, star_system::OSepster, unit::Zone};

/// Composition…
/// 
/// S - single star
/// B — a binary (or larger…)
/// T — a trinary (or larger…)
/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum StarSystemComposition {
    S,
    B {
        orbit_param: (OrbitalSeparation, OrbitalEccentricity),
        forbidden_zone: Zone,
        primary: Star,
        secondary: Box<StarSystemComposition>,
    },
    T {
        orbit_param: [(OrbitalSeparation, OrbitalEccentricity); 2],
        forbidden_zone: [Zone; 2],
        primary: Star,
        secondary: Box<StarSystemComposition>,
        tertiary: Box<StarSystemComposition>,
    },
}

impl StarSystemComposition {
    pub fn num_major_celestials(&self) -> usize {
        match self {
            Self::S => 1,
            Self::B {secondary,..} => 1 + secondary.num_major_celestials(),
            Self::T {secondary, tertiary,..} => 1 + secondary.num_major_celestials() + tertiary.num_major_celestials(),
        }
    }
}

pub(crate) enum BaseSystemComposition {
    S (Star),
    B { p: Star, s: OrbitalSeparation, e: OrbitalEccentricity, b: Box<BaseSystemComposition> },
    T { p: Star,
        s1: OrbitalSeparation, s2: OrbitalSeparation,
        e1: OrbitalEccentricity, e2: OrbitalEccentricity,
        b1: Box<BaseSystemComposition>, b2: Box<BaseSystemComposition>
        }
}

impl From<&OSepster> for BaseSystemComposition {
    fn from(oseps: &OSepster) -> Self {
        match oseps {
            OSepster::S(p) => BaseSystemComposition::S(Star::random("<unnamed>", p, &Zone::Free)),
            OSepster::B { p, s, b, e } => BaseSystemComposition::B {
                p: Star::random("<unnamed>", p, &Zone::from(e)),
                s: s.clone(), e: e.clone(),
                b: Box::new(BaseSystemComposition::from(&**b))
            },
            OSepster::T { p, s1, s2, e1, e2, b1, b2 } => BaseSystemComposition::T {
                p: Star::random("<unnamed>", p, &Zone::from(e1)),
                s1: s1.clone(), s2: s2.clone(),
                e1: e1.clone(), e2: e2.clone(),
                b1: Box::new(BaseSystemComposition::from(&**b1)),
                b2: Box::new(BaseSystemComposition::from(&**b2))
            }
        }
    }
}