use rand::Rng;

use crate::unit::distance::{Distance, Distanced};

use super::{OrbitElement, OrbitalInfo};

#[derive(Clone, Copy)]
enum Type {
    C, S, M,
}

impl Type {
    pub fn random() -> Type {
        let sr = rand::thread_rng().gen_range(15..=20);
        let r = rand::thread_rng().gen_range(1..=100);
        if r < 76 {
            Self::C
        } else if r <= 75 + sr {
            Self::S
        } else {
            Self::M
        }
    }
}

#[derive(Clone, Copy)]
pub struct AsteroidBelt {
    distance: Distance,
    r#type: Type,
}

impl OrbitalInfo for AsteroidBelt {
    fn distance(&self) -> Distance {
        self.distance
    }
}

impl AsteroidBelt {
    pub fn random(distance: Distance) -> OrbitElement {
        OrbitElement::AsteroidBelt(AsteroidBelt {
            distance,
            r#type: Type::random(),
        })
    }
}

impl std::fmt::Display for AsteroidBelt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} asteroid belt at {:.2}", match self.r#type {
            Type::C => "a C-type",
            Type::S => "an S-type",
            Type::M => "an M-type"
        }, self.distance())
    }
}
