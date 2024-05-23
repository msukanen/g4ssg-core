use dice::DiceExt;

pub(crate) enum SiliconBase {
    SulfuricAcid,
    LiquidSulfur,
    LiquidRock
}

pub(crate) enum ExoticaBase {
    NebulaDwelling,
    Machine,
    Magnetic
}

impl ExoticaBase {
    pub fn random() -> ExoticaBase {
        match 1.d3() {
            1 => ExoticaBase::NebulaDwelling,
            2 => ExoticaBase::Machine,
            _ => ExoticaBase::Magnetic
        }
    }
}

pub(crate) enum ChemicalBase {
    Hydrogen,
    Ammonia,
    Hydrocarbon,
    Water,
    Chlorine,
    Silicon(SiliconBase),
    Plasma,
    Exotica(ExoticaBase)
}

impl ChemicalBase {
    pub fn random() -> ChemicalBase {
        match 3.d6() {
            ..=5 => Self::Hydrogen,
            6|7 => Self::Ammonia,
            8 => Self::Hydrocarbon,
            9..=11 => Self::Water,
            12 => Self::Chlorine,
            13 => Self::Silicon(SiliconBase::SulfuricAcid),
            14 => Self::Silicon(SiliconBase::LiquidSulfur),
            15 => Self::Silicon(SiliconBase::LiquidRock),
            16 => Self::Plasma,
            _ => Self::Exotica(ExoticaBase::random())
        }
    }
}
