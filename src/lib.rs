//! Multiverse, universe, galaxy, and/or star system generator.
//! 
//! Applying GURPS 4E Space rules set (to some extent).
//! 
#![allow(unexpected_cfgs)]
pub mod celestial;
pub(crate) mod evo;
pub mod math;
pub mod star_system;
pub mod unit;

pub const UNNAMED: &'static str = "<unnamed>";
