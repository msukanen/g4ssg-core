use dicebag::DiceExt;

#[derive(Clone, PartialEq)]
pub enum MarginalComponent {
    Chlorine,
    Fluorine,
    HiCO2,
    HiO,
    InertGases,
    LowO,
    NitrogenCompounds,
    OrganicToxins,
    Pollutants,
    SulfurCompounds,
}

impl MarginalComponent {
    pub fn random() -> MarginalComponent {
        match 3.d6() {
            ..=4 => if 1.d6() < 4 {Self::Chlorine} else {Self::Fluorine},
            5|6 => Self::SulfurCompounds,
            7 => Self::NitrogenCompounds,
            8|9 => Self::OrganicToxins,
            10|11 => Self::LowO,
            12|13 => Self::Pollutants,
            14 => Self::HiCO2,
            15|16 => Self::HiO,
            17.. => Self::InertGases
        }
    }
}
