use std::fmt::Display;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{age::StellarPopulation, celestial::{orbital::{OSDMethod, OrbitalEccentricity, OrbitalSeparation, Zone}, star::Star, system_composition::{BaseSystemComposition, StarSystemComposition}}, unit::Zone};


/// "It's full of stars!", or at least one or a few…
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StarSystem {
    pub name: String,
    age: StellarPopulation,
    composition: StarSystemComposition,
}

impl StarSystem {
    /// Get number of major celestial objects in the system (stars, black holes, etc.).
    pub fn num_major_celestials(&self) -> usize {
        self.composition.num_major_celestials()
    }

    /// Generate a random star system.
    /// 
    /// # Args
    /// * `name`— ditto.
    /// * `icoc`— in core/cluster.
    pub fn random(name: &str, icoc: bool) -> Self {
        let age = StellarPopulation::random();

        let num_stars = match 3.d6() + if icoc {3} else {0} {
            ..=10 => 1,
            ..=15 => 2,
            _     => 3 // larger systems are possible, but that is determined by OSepster below.
        };

        // Lets get the separation scaffolding in place first…
        let oseps = match num_stars {
            1 => OSepster::S(age.clone()),
            2 => OSepster::random(&age, OSDMethod::Basic),
            3 => {
                let s1 = OrbitalSeparation::random(OSDMethod::TOB);
                let s2 = OrbitalSeparation::random(OSDMethod::TOB);
                let b1 = Box::new(if let OrbitalSeparation::D(_) = &s1 {
                    OSepster::random(&age, OSDMethod::SC)
                } else { OSepster::S(age.clone()) });
                let b2 = Box::new(if let OrbitalSeparation::D(_) = &s2 {
                    OSepster::random(&age, OSDMethod::SC)
                } else { OSepster::S(age.clone()) });
                let e1 = OrbitalEccentricity::random(&s1);
                let e2 = OrbitalEccentricity::random(&s2);
                OSepster::T { p: age.clone(), s1, s2, e1, e2, b1, b2 }
            }
            _ => unreachable!("Base `num_stars` is always 1, 2, or 3.")
        };

        // Plug in base stars…
        let stars = BaseSystemComposition::from(&oseps);

        Self {
            age,
            name: name.into(),
            composition: StarSystemComposition::default(),
        }
    }
}

/// Orbital scaffolding intermediate.
pub(crate) enum OSepster {
    S (StellarPopulation),
    B { p: StellarPopulation, s: OrbitalSeparation, e: OrbitalEccentricity, b: Box<OSepster> },
    T { p: StellarPopulation,
        s1: OrbitalSeparation, s2: OrbitalSeparation,
        e1: OrbitalEccentricity, e2: OrbitalEccentricity,
        b1: Box<OSepster>, b2: Box<OSepster>
        }
} impl OSepster {
    fn random(age: &StellarPopulation, method: OSDMethod) -> Self {
        let s = OrbitalSeparation::random(method);
        let b = Box::new(if let OrbitalSeparation::D(_) = &s {
            Self::random(age, OSDMethod::SC)
        } else {
            Self::S ( age.clone() )
        });
        let e = OrbitalEccentricity::random(&s);
        Self::B { p: age.clone(), s, b, e }
    }

    fn num_major_celestials(&self) -> usize {
        match self {
            Self::S(_) => 1,
            Self::B { b,.. } => 1 + b.num_major_celestials(),
            Self::T { b1, b2,.. } => 1 + b1.num_major_celestials() + b2.num_major_celestials()
        }
    }
}

#[cfg(test)]
    impl Display for OSepster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_inner(node: &OSepster, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
            let pad = " ".repeat(indent);
            match node {
                OSepster::S => writeln!(f, "{}S", pad)?,
                OSepster::B { s, b } => {
                    writeln!(f, "{}B({:?})", pad, s)?;
                    fmt_inner(b, f, indent + 2)?;
                },
                OSepster::T { s1, s2, b1, b2 } => {
                    writeln!(f, "{}T [={}]\n  {}> S", pad, node.num_major_celestials(), pad)?;
                    writeln!(f, "{}  ├─ s1 = {:?}", pad, s1)?;
                    fmt_inner(b1, f, indent + 4)?;
                    writeln!(f, "{}  └─ s2 = {:?}", pad, s2)?;
                    fmt_inner(b2, f, indent + 4)?;
                }
            }
            Ok(())
        }

        fmt_inner(self, f, 0)?;
        Ok(())
    }
}

#[cfg(test)]
mod osepster_tests {
    use super::*;
    #[test]
    fn oseps_flat_trinary_as_root() {
        let oseps = {
                let s1 = OrbitalSeparation::random(OSDMethod::TOB);
                let s2 = OrbitalSeparation::random(OSDMethod::TOB);
                let b1 = Box::new(if let OrbitalSeparation::D(_) = &s1 {
                    OSepster::random(OSDMethod::SC)
                } else { OSepster::S });
                let b2 = Box::new(if let OrbitalSeparation::D(_) = &s2 {
                    OSepster::random(OSDMethod::SC)
                } else { OSepster::S });
                OSepster::T { s1, s2, b1, b2 }
            };
        let _ = env_logger::try_init();
        log::debug!("\n{oseps}");
    }
}