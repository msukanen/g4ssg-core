//! Star System Specs
//! 
//! [StarSystem] struct acts as the "root" of everything in any given star system.
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{unit::age::StellarPopulation, celestial::{CountCelestialMajors, orbital::{OSDMethod, OrbitEccentricity, OrbitSeparation}, star::StarGenCtx, system_composition::Composition}};


/// "It's full of stars!", or at least one or a few…
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StarSystem {
    pub name: String,
    age: StellarPopulation,
    composition: Composition,
}

impl StarSystem {
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
        let scaffolding = match num_stars {
            1 => OrbitScaffolding::S(age.clone()),
            2 => OrbitScaffolding::random(&age, OSDMethod::Basic),
            _ => {
                let s1 = OrbitSeparation::random(OSDMethod::TOB);
                let s2 = OrbitSeparation::random(OSDMethod::TOB);
                let b1 = Box::new(if let OrbitSeparation::D(_) = &s1 {
                    OrbitScaffolding::random(&age, OSDMethod::SC)
                } else { OrbitScaffolding::S(age.clone()) });
                let b2 = Box::new(if let OrbitSeparation::D(_) = &s2 {
                    OrbitScaffolding::random(&age, OSDMethod::SC)
                } else { OrbitScaffolding::S(age.clone()) });
                let e1 = OrbitEccentricity::random(&s1);
                let e2 = OrbitEccentricity::random(&s2);
                OrbitScaffolding::T { p: age.clone(), s1, s2, e1, e2, b1, b2 }
            }
        };

        // Now, with the scaffolding, plug in the stars and their stuff.
        let mut genctx = StarGenCtx::default();
        let composition = Composition::from( ScaffoldingCtx { scaffolding: &scaffolding, genctx: &mut genctx });
        // TODO: binary interactions, novae effects?

        Self {
            age,
            name: name.into(),
            composition,
        }
    }
}

impl CountCelestialMajors for StarSystem {
    fn num_major_celestials(&self) -> usize {
        self.composition.num_major_celestials()
    }
}

/// Orbital scaffolding intermediate.
pub(crate) enum OrbitScaffolding {
    S (StellarPopulation),
    B { p: StellarPopulation, s: OrbitSeparation, e: OrbitEccentricity, b: Box<OrbitScaffolding> },
    T { p: StellarPopulation,
        s1: OrbitSeparation, s2: OrbitSeparation,
        e1: OrbitEccentricity, e2: OrbitEccentricity,
        b1: Box<OrbitScaffolding>, b2: Box<OrbitScaffolding>
        }
} impl OrbitScaffolding {
    /// Generate random specs for an orbital separation branch.
    fn random(age: &StellarPopulation, method: OSDMethod) -> Self {
        let s = OrbitSeparation::random(method);
        let b = Box::new(if let OrbitSeparation::D(_) = &s {
            Self::random(age, OSDMethod::SC)
        } else {
            Self::S ( age.clone() )
        });
        let e = OrbitEccentricity::random(&s);
        Self::B { p: age.clone(), s, b, e }
    }

    #[cfg(test)]
    fn num_major_celestials(&self) -> usize {
        match self {
            Self::S(_) => 1,
            Self::B { b,.. } => 1 + b.num_major_celestials(),
            Self::T { b1, b2,.. } => 1 + b1.num_major_celestials() + b2.num_major_celestials()
        }
    }
}

pub(crate) struct ScaffoldingCtx<'a> {
    pub scaffolding: &'a OrbitScaffolding,
    pub genctx: &'a mut StarGenCtx,
}

#[cfg(test)]
mod osepster_tests {
    use super::*;

    impl std::fmt::Display for OrbitScaffolding {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fn fmt_inner(node: &OrbitScaffolding, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
                let pad = " ".repeat(indent);
                match node {
                    OrbitScaffolding::S(_) => writeln!(f, "{}S", pad)?,
                    OrbitScaffolding::B { s, b, .. } => {
                        writeln!(f, "{}B({:?})", pad, s)?;
                        fmt_inner(b, f, indent + 2)?;
                    },
                    OrbitScaffolding::T { s1, s2, b1, b2, .. } => {
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

    #[test]
    fn oseps_flat_trinary_as_root() {
        let age = StellarPopulation::random();
        let oseps = {
                let s1 = OrbitSeparation::random(OSDMethod::TOB);
                let s2 = OrbitSeparation::random(OSDMethod::TOB);
                let b1 = Box::new(if let OrbitSeparation::D(_) = &s1 {
                    OrbitScaffolding::random(&age, OSDMethod::SC)
                } else { OrbitScaffolding::S(age.clone()) });
                let b2 = Box::new(if let OrbitSeparation::D(_) = &s2 {
                    OrbitScaffolding::random(&age, OSDMethod::SC)
                } else { OrbitScaffolding::S(age.clone()) });
                let e1 = OrbitEccentricity::random(&s1);
                let e2 = OrbitEccentricity::random(&s2);
                OrbitScaffolding::T { p: age.clone(), s1, s2, b1, b2, e1, e2 }
            };
        let _ = env_logger::try_init();
        log::debug!("\n{oseps}");
    }
}