use dice::DiceExt;

use super::{habitat::Habitat, symmetry::Symmetry};

pub enum Feature {
    Simple,
    Striker(bool),
    Long,
    Constricting,
    Gripping,
    Branching,
}

impl std::fmt::Display for Feature {
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

impl Feature {
    pub fn random(habitat: &Habitat, symmetry: &Symmetry) -> Vec<Feature> {
        if 1.d6() < 4 || match symmetry {
            Symmetry::Spherical(_) => return vec![],
            _ => false
        } {
            return vec![];
        }

        fn select(habitat: &Habitat) -> Vec<Feature> {
            let mut features = vec![];
            match 2.d6() {
                ..=5 => features.push(Feature::Simple),
                6 => features.push(Feature::Striker(false)),
                7 => features.push(Feature::Long),
                8 => features.push(Feature::Constricting),
                9 => features.push(Feature::Striker(true)),
                10 => features.push(Feature::Gripping),
                11 => features.push(Feature::Branching),
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
