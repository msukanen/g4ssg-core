use dicebag::DiceExt;

use crate::life::symmetry::Symmetry;

pub enum NumberOfLimbs {
    Spherical(i32),
    Asymmetric(i32),
    Bilateral(i32, i32),
    Trilateral(i32, i32),
    Radial(i32, i32),
}

impl NumberOfLimbs {
    pub fn random(symmetry: &Symmetry) -> NumberOfLimbs {
        let modifier = match symmetry {
            Symmetry::Trilateral => -1,
            Symmetry::Radial(_) => -2,
            _ => 0
        };

        fn btr_limbs(modifier: i32) -> (i32, i32) {
            match 1.d6() + modifier {
                ..=1 => (0,0),
                2 => (1,1),
                3 => (2,2),
                4 => (1.d6(),1),
                5 => (2.d6(),1),
                6.. => (3.d6(),1)
            }
        }

        match symmetry {
            Symmetry::Spherical(x) => Self::Spherical(*x),
            Symmetry::Asymmetric => Self::Asymmetric(2.d6() - 2),
            _ => {
                let (segments, limbs_per_side) = btr_limbs(modifier);
                match symmetry {
                    Symmetry::Trilateral => Self::Trilateral(segments, limbs_per_side * 3),
                    Symmetry::Radial(n) => Self::Radial(segments, limbs_per_side * n),
                    _ => Self::Bilateral(segments, limbs_per_side * 2)
                }
            }
        }
    }

    pub fn count(&self) -> i32 {
        match self {
            Self::Asymmetric(n) |
            Self::Spherical(n)  => *n,
            Self::Bilateral(s, c) |
            Self::Trilateral(s, c)|
            Self::Radial(s, c)    => *s * *c
        }
    }
}
