use super::population::Population;

pub enum EvolutionStage {
    M, S, G, D
}

impl From<(&Population, (Option<f64>, Option<f64>, Option<f64>))> for EvolutionStage {
    fn from(value: (&Population, (Option<f64>, Option<f64>, Option<f64>))) -> Self {
        let (p, (m, s, g)) = value;
        if let Some(m) = m {
            if p.byr() <= m {
                Self::M
            } else if let Some(s) = s {
                if p.byr() <= m + s {
                    Self::S
                } else if let Some(g) = g {
                    if p.byr() <= m + s + g {
                        Self::G
                    } else {
                        Self::D
                    }
                } else {
                    Self::D
                }
            } else {
                Self::D
            }
        } else {
            Self::M
        }
    }
}
