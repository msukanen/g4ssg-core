use super::{evolutionstage::EvolutionStage, measurement::massindex::{MAX_MASS_INDEX, MIN_MASS_INDEX}};

#[derive(Debug, PartialEq)]
pub enum Type {
    D,
    M(i32),
    K(i32),
    G(i32),
    F(i32),
    A(i32),
}

impl From<(i32, &EvolutionStage)> for Type {
    /**
     Generate approximate `Type` from the given mass index.
     */
    fn from(value: (i32, &EvolutionStage)) -> Self {
        if value.1 == &EvolutionStage::D {
            return Self::D;
        }

        let mass_index = value.0.clamp(MIN_MASS_INDEX, MAX_MASS_INDEX);
        match mass_index {
            ..=MIN_MASS_INDEX => Self::A(5),
            1 => Self::A(6),
            2 => Self::A(7),
            3 => Self::A(9),
            4 => Self::F(0),
            5 => Self::F(2),
            6 => Self::F(3),
            7 => Self::F(4),
            8 => Self::F(5),
            9 => Self::F(6),
            10 => Self::F(7),
            11 => Self::F(8),
            12 => Self::F(9),
            13 => Self::G(0),
            14 => Self::G(1),
            15 => Self::G(2),
            16 => Self::G(4),
            17 => Self::G(6),
            18 => Self::G(8),
            19 => Self::K(0),
            20 => Self::K(2),
            21 => Self::K(4),
            22 => Self::K(5),
            23 => Self::K(6),
            24 => Self::K(8),
            25 => Self::M(0),
            26 => Self::M(1),
            27 => Self::M(2),
            28 => Self::M(3),
            29|30 => Self::M(4),
            31 => Self::M(5),
            32 => Self::M(6),
            MAX_MASS_INDEX.. => Self::M(7),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::A(a) => format!("A{a}"),
            Self::F(a) => format!("F{a}"),
            Self::G(a) => format!("G{a}"),
            Self::K(a) => format!("K{a}"),
            Self::M(a) => format!("M{a}"),
            Self::D => "".to_string(),
        })
    }
}

#[cfg(test)]
mod type_test {
    use crate::starsystem::orbital::star::{evolutionstage::EvolutionStage, measurement::massindex::MAX_MASS_INDEX};

    use super::Type;

    #[test]
    pub fn negative_mass_index_treated_as_zero() {
        let t0 = Type::from((0, &EvolutionStage::M));
        let tn = Type::from((-100, &EvolutionStage::M));
        assert_eq!(t0, Type::A(5));
        assert_eq!(tn, t0);
    }

    #[test]
    pub fn too_large_mass_index_treated_as_max_mass_index() {
        let t0 = Type::from((MAX_MASS_INDEX, &EvolutionStage::M));
        let tn = Type::from((1000, &EvolutionStage::M));
        assert_eq!(t0, Type::M(7));
        assert_eq!(tn, t0);
    }
}
