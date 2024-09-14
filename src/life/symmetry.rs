use dicebag::DiceExt;

use super::{habitat::Habitat, locomotion::{FlightMode, Locomotion, LocomotionMode}};

#[derive(PartialEq)]
pub enum Symmetry {
    Bilateral,
    Trilateral,
    Radial(i32),
    Spherical(i32),
    Asymmetric
}

impl std::fmt::Display for Symmetry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Asymmetric => "asymmetric".to_string(),
            Self::Bilateral => "bilateral".to_string(),
            Self::Radial(x) => format!("radial with {x} sides"),
            Self::Spherical(x) => format!("spherical with {x} \"sides\""),
            Self::Trilateral => "trilateral".to_string()
        })
    }
}

impl Symmetry {
    pub fn random(habitat: &Habitat, locomotion: &Locomotion) -> Symmetry {
        let modifier = if locomotion.is(LocomotionMode::Flight(FlightMode::Buoyant))
            || locomotion.is_immobile()
            || match habitat {
                Habitat::Space => true,
                _ => false
            }
        {1} else {0};
        match 2.d6() + modifier {
            ..=7 => Self::Bilateral,
            8 => Self::Trilateral,
            9 => Self::Radial(1.d6() + 3),
            10 => Self::Spherical(match 1.d6() {
                ..=1 => 4,
                2|3 => 6,
                4 => 8,
                5 => 12,
                6.. => 20,
            }),
            11.. => Self::Asymmetric
        }
    }

    pub fn is(&self, symmetry: &Symmetry) -> bool {
        self == symmetry
    }
}
