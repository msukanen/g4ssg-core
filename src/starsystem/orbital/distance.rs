use dicebag::DiceExt;

use crate::unit::distance::{au::Au, Distance};

use super::separation::OrbitalSeparation;

#[derive(Clone, Copy)]
pub struct OrbitalDistance {
    separation: OrbitalSeparation,
    eccentricity: f64,
    step: i32,
}

impl std::fmt::Display for OrbitalDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OD {}, {}, {}", self.separation, self.eccentricity, self.step)
    }
}

impl OrbitalDistance {
    pub fn random(separation: &OrbitalSeparation) -> OrbitalDistance {
        OrbitalDistance {
            separation: *separation,
            eccentricity: match 3.d6() + separation.eccentricity_modifier() {
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
                18.. => 0.95
            },
            step: 2.d6(),
        }
    }

    pub fn eccentricity(&self) -> f64 {
        self.eccentricity
    }

    pub fn step(&self) -> f64 {
        self.step as f64
    }

    pub fn separation(&self) -> &OrbitalSeparation {
        &self.separation
    }

    pub fn min(&self) -> Distance {
        Distance::Au(Au::from((1.0 - self.eccentricity()) * self.average()))
    }

    pub fn max(&self) -> Distance {
        Distance::Au(Au::from((1.0 + self.eccentricity()) * self.average()))
    }

    pub fn average(&self) -> Distance {
        Distance::Au(Au::from(self.step() * self.separation().radius_multiplier()))
    }
}
