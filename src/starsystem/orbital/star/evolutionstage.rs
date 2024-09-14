use super::population::Population;

#[derive(PartialEq)]
pub enum EvolutionStage {
    /// Main Sequence
    M,
    /// Subgiant
    S,
    /// Giant (generally III).
    G,
    /// White dwarf or other remnant of a dead star.
    Dead
}

impl From<(&Population, (Option<f64>, Option<f64>, Option<f64>))> for EvolutionStage {
    /**
     Make [evolution stage][EvolutionStage] from [Population] and age range (tuple)
     that represents time spent as Main Sequence, Subgiant and Giant.
     */
    fn from(value: (&Population, (Option<f64>, Option<f64>, Option<f64>))) -> Self {
        if let Some(m) = value.1.0 {
            // Below M-cap?
            if value.0.byr() <= m {
                Self::M
            } else if let Some(s) = value.1.1 {
                // Below S-cap?
                if value.0.byr() <= m + s {
                    Self::S
                } else if let Some(g) = value.1.2 {
                    // Are we still alive...?
                    if value.0.byr() <= m + s + g {
                        Self::G
                    } else {
                        Self::Dead
                    }
                } else {
                    Self::Dead
                }
            } else {
                Self::Dead
            }
        } else {
            Self::M
        }
    }
}

impl std::fmt::Display for EvolutionStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Dead => "WD",// white dwarf
            Self::G => "III",
            Self::M => "V",
            Self::S => "IV",
        })
    }
}
