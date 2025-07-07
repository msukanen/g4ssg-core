use dicebag::DiceExt;

use crate::core::orbit::separation::OrbitalSeparation;

pub fn gen_eccentricity_ratio(separation: &OrbitalSeparation) -> f64 {
    separation.new_ratio()
}

pub trait OrbitalEccentricityExt {
    fn new_ratio(&self) -> f64;
}

impl OrbitalEccentricityExt for OrbitalSeparation {
    fn new_ratio(&self) -> f64 {
        match 3.d6() + match self {
            Self::VeryClose => -6,
            Self::Close => -4,
            Self::Moderate => -2,
            _ => 0
        } {
            ..=3 => 0.0,
            4 => 0.1,
            5 => 0.2,
            6 => 0.3,
            7|8 => 0.4,
            9..=11 => 0.5,
            12|13 => 0.6,
            14|15 => 0.7,
            16 => 0.8,
            17 => 0.9,
            _ => 0.95
        }
    }
}
