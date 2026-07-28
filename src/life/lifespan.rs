//! Lifespan.

use std::cmp::Ordering;

use astrometrics::AsSpatialUnit;
use serde::{Deserialize, Serialize};

use crate::life::{chemistry::{ChemicalBasis, ExoticBase}, habitat::{Habitat, LandHabitat}, size::Size};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Ord)]
pub enum Lifespan {
    Short { grade: u8 },
    Normal,
    Extended { grade: u8 },
    Immortal,
}

impl PartialOrd for Lifespan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Short { grade: g1 }, Self::Short { grade: g2 }) => g2.partial_cmp(g1),
            (Self::Short { .. }, _) => Ordering::Less.into(),
            (Self::Normal, Self::Short { .. }) => Ordering::Greater.into(),
            (Self::Normal, Self::Normal) => Ordering::Equal.into(),
            (Self::Normal, _) => Ordering::Less.into(),
            (Self::Extended { grade: g1 }, Self::Extended { grade: g2 }) => g1.partial_cmp(g2),
            (Self::Extended { .. }, Self::Immortal) => Ordering::Less.into(),
            (Self::Extended { .. }, _) => Ordering::Greater.into(),
            (Self::Immortal, Self::Immortal) => Ordering::Equal.into(),
            (Self::Immortal, _) => Ordering::Greater.into()
        }
    }
}

impl Lifespan {
    pub fn derive(sapient: bool, cb: ChemicalBasis, size: Size, habitat: Habitat) -> Self {
        let mut span = match size {
            Size::Small { hl, .. }     |
            Size::HumanScale { hl, ..} |
            Size::Large { hl, .. }     =>
                match hl {
                    _ if hl < 0.3.m() => Self::Short { grade: 4 },
                    _ if hl < 0.75.m() => Self::Short { grade: 3 },
                    _ if hl < 1.35.m() => Self::Short { grade: 2 },
                    _ if hl < 2.7.m() => Self::Short { grade: 1 },
                    _ if hl > 100.m() => Self::Extended { grade: 3 },
                    _ if hl > 30.m() => Self::Extended { grade: 2 },
                    _ if hl > 6.1.m() => Self::Extended { grade: 1 },
                    _ => Self::Normal
                }
        };
        
        match cb {
            ChemicalBasis::Ammonia  |
            ChemicalBasis::Hydrogen => span.extend(),
            
            ChemicalBasis::Exotic(ExoticBase::Magnetic) |
            ChemicalBasis::Silicon(_) => span.shorten(),
            
            _ => ()
        }

        match habitat {
            Habitat::Land(LandHabitat::Arctic) => span.extend(),
            Habitat::Space(_) => { span.extend(); span.extend(); },
            _ => ()
        }
        
        if sapient {
            span.extend()
        }
        
        span
    }

    pub const fn next(&self) -> Self {
        match self {
            Self::Short { grade } => {
                let grade = *grade - 1;
                if grade > 0 {
                    Self::Short { grade }
                } else {
                    Self::Normal
                }
            },
            Self::Normal => Self::Extended { grade: 1 },
            Self::Extended { grade } => {
                let grade = *grade + 1;
                if grade > 4 {
                    Self::Immortal
                } else {
                    Self::Extended { grade }
                }
            },
            _ => *self
        }
    }

    pub const fn prev(&self) -> Self {
        match self {
            // Once immortal, always immortal…
            Self::Immortal => *self,
            Self::Extended { grade } => {
                let grade = *grade - 1;
                if grade > 0 {
                    Self::Extended { grade }
                } else {
                    Self::Normal
                }
            },
            Self::Normal => Self::Short { grade: 1 },
            Self::Short { grade } => Self::Short { grade: *grade + 1 }
        }
    }

    pub fn extend(&mut self) { *self = self.next() }
    pub fn shorten(&mut self) { *self = self.prev() }
}
