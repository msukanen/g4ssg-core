use dice::DiceExt;

use super::{habitat::Habitat, symmetry::Symmetry};

pub enum Tail {
    Simple,
    Striker(bool),
    Long,
    Constricting,
    Gripping,
    Branching,
}

impl std::fmt::Display for Tail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Branching => "branching (splits according to body symmetry)".to_string(),
            Self::Constricting => "constricting".to_string(),
            Self::Gripping => "gripping (counts as an Extra Arm with Bad Grip)".to_string(),
            Self::Long => "long".to_string(),
            Self::Simple => "featureless".to_string(),
            Self::Striker(barbed) => format!("{}striker", if *barbed {"barbed "} else {""})
        })
    }
}

impl Tail {
    pub fn random(habitat: &Habitat, symmetry: &Symmetry) -> Vec<Tail> {
        if 1.d6() < 4 || match symmetry {
            Symmetry::Spherical(_) => return vec![],
            _ => false
        } {
            return vec![];
        }

        fn select(habitat: &Habitat) -> Vec<Tail> {
            let mut features = vec![];
            match 2.d6() {
                ..=5 => features.push(Tail::Simple),
                6 => features.push(Tail::Striker(false)),
                7 => features.push(Tail::Long),
                8 => features.push(Tail::Constricting),
                9 => features.push(Tail::Striker(true)),
                10 => features.push(Tail::Gripping),
                11 => features.push(Tail::Branching),
                _ => {
                    features.extend(select(habitat));
                    features.extend(select(habitat))
                }
            }
            features
        }
        
        select(habitat)
    }
}
