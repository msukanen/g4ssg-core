use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{celestial::{CountCelestialMajors, orbital::{OrbitEccentricity, OrbitSeparation}, star::Star}, star_system::{OrbitScaffolding, ScaffoldingCtx}, unit::Zone};

/// Composition…
/// 
/// S - single star
/// B — a binary (or larger…)
/// T — a trinary (or larger…)
/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Composition {
    S (Star),
    B { p: Star, s: OrbitSeparation, e: OrbitEccentricity, b: Box<Composition> },
    T { p: Star,
        s1: OrbitSeparation, s2: OrbitSeparation,
        e1: OrbitEccentricity, e2: OrbitEccentricity,
        b1: Box<Composition>, b2: Box<Composition>
        }
}

impl<'a> From<ScaffoldingCtx<'a>> for Composition {
    fn from(ctx: ScaffoldingCtx<'a>) -> Self {
        match ctx.scaffolding {
            OrbitScaffolding::S(p) => Composition::S(Star::random("<unnamed>", p, &Zone::Free, ctx.genctx)),
            OrbitScaffolding::B { p, s, b, e } => Composition::B {
                p: Star::random("<unnamed>", p, &Zone::from(e), ctx.genctx),
                s: s.clone(), e: e.clone(),
                b: Box::new(Composition::from( ScaffoldingCtx { scaffolding: &**b, genctx: ctx.genctx }))
            },
            OrbitScaffolding::T { p, s1, s2, e1, e2, b1, b2 } => Composition::T {
                p: Star::random("<unnamed>", p, &Zone::from(e1), ctx.genctx),
                s1: s1.clone(), s2: s2.clone(),
                e1: e1.clone(), e2: e2.clone(),
                b1: Box::new(Composition::from( ScaffoldingCtx{ scaffolding: &**b1, genctx: ctx.genctx })),
                b2: Box::new(Composition::from( ScaffoldingCtx{ scaffolding: &**b2, genctx: ctx.genctx }))
            }
        }
    }
}

impl Composition {
    fn iter(&self) -> CompositionIter<'_> {
        let mut stack = VecDeque::new();
        stack.push_back(self);
        CompositionIter { stack }
    }
}

impl CountCelestialMajors for Composition {
    fn num_major_celestials(&self) -> usize {
        match self {
            Composition::S(_) => 1,
            Composition::B { b,.. } => 1 + b.num_major_celestials(),
            Composition::T { b1, b2,.. } => 1 + b1.num_major_celestials() + b2.num_major_celestials()
        }
    }
}

pub struct CompositionIter<'a> {
    stack: VecDeque<&'a Composition>,
}

impl <'a> Iterator for CompositionIter<'a> {
    type Item = &'a Composition;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop_front() {
            match node {
                Composition::S(_) => {},
                Composition::B { b, .. } => {
                    self.stack.push_back(&**b);
                },
                Composition::T { b1, b2, .. } => {
                    self.stack.push_back(&**b1);
                    self.stack.push_back(&**b2);
                }
            }
            Some(node)
        } else {
            None
        }
    }
}

impl <'a> IntoIterator for &'a Composition {
    type Item = &'a Composition;
    type IntoIter = CompositionIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CompositionIter { stack: VecDeque::new() }
    }
}