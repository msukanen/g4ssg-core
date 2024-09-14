use dicebag::DiceExt;

#[derive(PartialEq, Clone, Copy)]
pub enum Generation {
    I,
    II,
}

impl std::fmt::Display for Generation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::I  => "population I",
            Self::II => "population II"
        })
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Population {
    Extreme(Generation, f64),
    Young(f64),
    Intermediate(Generation, f64),
    Old(f64),
}

impl Population {
    pub fn random() -> Population {
        3.d6().as_population()
    }

    pub fn byr(&self) -> f64 {
        match self {
            Self::Extreme(Generation::I,_)   => 0.0,
            Self::Old(byr)             |
            Self::Young(byr)           |
            Self::Extreme(_, byr)      |
            Self::Intermediate(_, byr) => *byr,
        }
    }
}

pub trait AsPopulation {
    fn as_population(&self) -> Population;
}

fn age_from(base: f64, step_a: f64, step_b: f64) -> f64 {
    base
    + (1.d6()-1) as f64 * step_a
    + (1.d6()-1) as f64 * step_b
}

impl AsPopulation for i32 {
    fn as_population(&self) -> Population {
        match self {
            ..=3 => Population::Extreme(Generation::I, 0.0),
            4..=6 => Population::Young(age_from(0.1, 0.3, 0.05)),
            7..=10 => Population::Intermediate(Generation::I, age_from(2.0, 0.6, 0.1)),
            11..=14 => Population::Old(age_from(5.6, 0.6, 0.1)),
            15..=17 => Population::Intermediate(Generation::II, age_from(8.0, 0.6, 0.1)),
            18.. => Population::Extreme(Generation::II, age_from(10.0, 0.6, 0.1))
        }
    }
}

impl std::fmt::Display for Population {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Extreme(g, a) => format!("extreme {g} at {a:.1} byr"),
            Self::Intermediate(g, a) => format!("intermediate {g} at {a:.1} byr"),
            Self::Old(a) => format!("old population II at {a:.1} byr"),
            Self::Young(a) => format!("young population I at {a:.1} byr"),
        })
    }
}
