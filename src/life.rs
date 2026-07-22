//! Life, as we know it — and otherwise.

use crate::life::{chemistry::ChemicalBasis, habitat::Habitat, trophics::TrophicLevel};

pub mod chemistry;
pub mod habitat;
pub mod trophics;

pub struct Life {
    chemical_basis: ChemicalBasis,
    habitat: Habitat,
    trophic: TrophicLevel,
    locomotion: (Locomotion, Locomotion),
} 
