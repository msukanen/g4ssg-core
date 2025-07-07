//! Multiverse, universe, galaxy, and/or star system generator.
//! 
//! Applying GURPS 4E Space rules set (to some extent).
//! 
//! Author: `Markku Sukanen` <markku.sukanen@gmail.com>
//! 
//! [`Markku Sukanen`]: https://msukanen.net
#![allow(unexpected_cfgs)]
use crate::core::{galaxy::Galaxy, multiverse::Multiverse, starsystem::StarSystem, universe::Universe};
pub mod core;
mod util;

/// Generate an entire multiverse.
pub fn let_there_be_light() -> Multiverse {
    Multiverse::let_there_be_light()
}

/// Generate a single universe.
pub fn single_big_bang() -> Box<Universe> {
    Box::new(Universe::big_bang(0, 0))
}

/// Generate a single galaxy.
pub fn single_new_galaxy(designation: &str) -> Box<Galaxy> {
    Box::new(Galaxy::new(0, 0, Some(designation)))
}

/// Generate a single star system.
pub fn single_new_star_system(icoc: bool, designation: &str) -> Box<StarSystem> {
    Box::new(StarSystem::new(0, 0, icoc, Some(designation)))
}
